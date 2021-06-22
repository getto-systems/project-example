use jsonwebtoken::{decode, errors::ErrorKind, Algorithm, DecodingKey, Validation};

use crate::auth::auth_ticket::_api::{
    kernel::infra::{AuthJwtClaims, AUTH_JWT_AUDIENCE_API, AUTH_JWT_AUDIENCE_TICKET},
    validate::infra::AuthTokenDecoder,
};

use crate::auth::auth_ticket::_api::{
    kernel::data::{AuthTicket, AuthTokenValue},
    validate::data::DecodeAuthTokenError,
};

pub struct JwtAuthTokenDecoder<'a> {
    key: &'a DecodingKey<'a>,
}

impl<'a> JwtAuthTokenDecoder<'a> {
    pub const fn new(key: &'a DecodingKey) -> Self {
        Self { key }
    }
}

impl<'a> AuthTokenDecoder for JwtAuthTokenDecoder<'a> {
    fn decode(&self, token: &AuthTokenValue) -> Result<AuthTicket, DecodeAuthTokenError> {
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
    fn decode(&self, token: &AuthTokenValue) -> Result<AuthTicket, DecodeAuthTokenError> {
        validate_jwt(token, &[AUTH_JWT_AUDIENCE_API], &self.key)
    }
}

fn validate_jwt(
    token: &AuthTokenValue,
    audience: &[&str],
    key: &DecodingKey,
) -> Result<AuthTicket, DecodeAuthTokenError> {
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
    use crate::auth::auth_ticket::_api::validate::infra::AuthTokenDecoder;

    use crate::auth::auth_ticket::_api::{
        kernel::data::{AuthTicket, AuthTokenValue},
        validate::data::DecodeAuthTokenError,
    };

    pub enum StaticAuthTokenDecoder {
        Valid(AuthTicket),
        Expired,
    }

    impl AuthTokenDecoder for StaticAuthTokenDecoder {
        fn decode(&self, _token: &AuthTokenValue) -> Result<AuthTicket, DecodeAuthTokenError> {
            match self {
                Self::Expired => Err(DecodeAuthTokenError::Expired),
                Self::Valid(ticket) => Ok(ticket.clone()),
            }
        }
    }
}
