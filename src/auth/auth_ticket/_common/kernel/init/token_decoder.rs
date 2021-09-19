use jsonwebtoken::{decode, errors::ErrorKind, Algorithm, DecodingKey, Validation};

use crate::auth::auth_ticket::_common::kernel::infra::{
    AuthJwtClaims, AuthTokenDecoder, AUTH_JWT_AUDIENCE_API, AUTH_JWT_AUDIENCE_TICKET,
};

use crate::auth::auth_ticket::_common::kernel::data::{
    AuthTicketExtract, AuthToken, DecodeAuthTokenError,
};

pub struct JwtTicketTokenDecoder<'a> {
    key: &'a DecodingKey<'a>,
}

impl<'a> JwtTicketTokenDecoder<'a> {
    pub const fn new(key: &'a DecodingKey<'a>) -> Self {
        Self { key }
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
    pub const fn new(key: &'a DecodingKey<'a>) -> Self {
        Self { key }
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

#[cfg(test)]
pub mod test {
    use crate::auth::auth_ticket::_common::kernel::infra::AuthTokenDecoder;

    use crate::auth::auth_ticket::_common::kernel::data::{
        AuthTicketExtract, AuthToken, DecodeAuthTokenError,
    };

    pub enum StaticAuthTokenDecoder {
        Valid(AuthTicketExtract),
        Expired,
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
