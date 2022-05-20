use std::collections::HashSet;

use jsonwebtoken::{decode, errors::ErrorKind, Algorithm, DecodingKey, Validation};

use crate::auth::x_outside_feature::feature::AuthOutsideDecodingKey;

use crate::auth::ticket::{
    kernel::infra::{AuthJwtClaims, AUTH_JWT_AUDIENCE_API, AUTH_JWT_AUDIENCE_TICKET},
    validate::infra::AuthTokenDecoder,
};

use crate::auth::ticket::kernel::data::{AuthTicketExtract, AuthToken, DecodeAuthTokenError};

pub struct JwtAuthTokenDecoder<'a> {
    key: &'a DecodingKey,
    audience: [&'static str; 1],
}

impl<'a> JwtAuthTokenDecoder<'a> {
    pub const fn ticket(decoding_key: &'a AuthOutsideDecodingKey) -> Self {
        Self {
            key: &decoding_key.ticket,
            audience: [AUTH_JWT_AUDIENCE_TICKET],
        }
    }
    pub const fn api(decoding_key: &'a AuthOutsideDecodingKey) -> Self {
        Self {
            key: &decoding_key.api,
            audience: [AUTH_JWT_AUDIENCE_API],
        }
    }
}

impl<'a> AuthTokenDecoder for JwtAuthTokenDecoder<'a> {
    fn decode(&self, token: &AuthToken) -> Result<AuthTicketExtract, DecodeAuthTokenError> {
        validate_jwt(token, &self.audience, &self.key)
    }
}

fn validate_jwt(
    token: &AuthToken,
    audience: &[&str],
    key: &DecodingKey,
) -> Result<AuthTicketExtract, DecodeAuthTokenError> {
    let mut validation = Validation::new(Algorithm::ES384);
    validation.set_audience(audience);

    let data =
        decode::<AuthJwtClaims>(token.as_str(), &key, &validation).map_err(|err| {
            match err.kind() {
                ErrorKind::ExpiredSignature => DecodeAuthTokenError::Expired,
                _ => DecodeAuthTokenError::Invalid(format!("{}", err)),
            }
        })?;

    Ok(data.claims.into())
}

pub struct NoopTokenDecoder;

impl<'a> AuthTokenDecoder for NoopTokenDecoder {
    fn decode(&self, _token: &AuthToken) -> Result<AuthTicketExtract, DecodeAuthTokenError> {
        Ok(AuthTicketExtract {
            ticket_id: "noop-decoder".into(),
            user_id: "noop-decoder".into(),
            granted_roles: HashSet::new(),
        })
    }
}

#[cfg(test)]
pub mod test {
    use std::collections::HashSet;

    use crate::auth::ticket::validate::infra::AuthTokenDecoder;

    use crate::auth::ticket::kernel::data::{AuthTicketExtract, AuthToken, DecodeAuthTokenError};

    pub enum StaticAuthTokenDecoder {
        Valid(AuthTicketExtract),
        Expired,
    }

    impl StaticAuthTokenDecoder {
        pub fn valid(ticket_id: String, user_id: String, granted_roles: HashSet<String>) -> Self {
            Self::Valid(AuthTicketExtract {
                ticket_id,
                user_id,
                granted_roles,
            })
        }
    }

    impl AuthTokenDecoder for StaticAuthTokenDecoder {
        fn decode(&self, _token: &AuthToken) -> Result<AuthTicketExtract, DecodeAuthTokenError> {
            match self {
                Self::Expired => Err(DecodeAuthTokenError::Expired),
                Self::Valid(ticket) => Ok(ticket.clone()),
            }
        }
    }
}
