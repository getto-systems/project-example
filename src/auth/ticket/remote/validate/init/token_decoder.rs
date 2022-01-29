use std::collections::HashSet;

use jsonwebtoken::{decode, errors::ErrorKind, Algorithm, DecodingKey, Validation};

use crate::auth::remote::x_outside_feature::common::feature::AuthOutsideDecodingKey;

use crate::auth::ticket::remote::{
    kernel::infra::{AuthJwtClaims, AUTH_JWT_AUDIENCE_API, AUTH_JWT_AUDIENCE_TICKET},
    validate::infra::AuthTokenDecoder,
};

use crate::auth::ticket::remote::kernel::data::{
    AuthTicketExtract, AuthToken, DecodeAuthTokenError,
};

pub struct JwtTicketTokenDecoder<'a> {
    key: &'a DecodingKey<'a>,
}

impl<'a> JwtTicketTokenDecoder<'a> {
    pub const fn new(key: &'a AuthOutsideDecodingKey) -> Self {
        Self { key: &key.ticket }
    }
}

impl<'a> AuthTokenDecoder for JwtTicketTokenDecoder<'a> {
    fn decode(&self, token: &AuthToken) -> Result<AuthTicketExtract, DecodeAuthTokenError> {
        validate_jwt(token, &[AUTH_JWT_AUDIENCE_TICKET], &self.key)
    }
}

pub struct JwtApiTokenDecoder<'a> {
    key: &'a DecodingKey<'a>,
}

impl<'a> JwtApiTokenDecoder<'a> {
    pub const fn new(key: &'a AuthOutsideDecodingKey) -> Self {
        Self { key: &key.api }
    }
}

impl<'a> AuthTokenDecoder for JwtApiTokenDecoder<'a> {
    fn decode(&self, token: &AuthToken) -> Result<AuthTicketExtract, DecodeAuthTokenError> {
        validate_jwt(token, &[AUTH_JWT_AUDIENCE_API], &self.key)
    }
}

fn validate_jwt<'a>(
    token: &AuthToken,
    audience: &[&str],
    key: &DecodingKey<'a>,
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

    use crate::auth::ticket::remote::validate::infra::AuthTokenDecoder;

    use crate::auth::ticket::remote::kernel::data::{
        AuthTicketExtract, AuthToken, DecodeAuthTokenError,
    };

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
