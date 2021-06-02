use jsonwebtoken::{decode, errors::ErrorKind, Algorithm, DecodingKey, Validation};

use super::super::super::kernel::infra::{AuthJwtClaims, AUTH_JWT_AUDIENCE_API, AUTH_JWT_AUDIENCE_TICKET};

use super::AuthTokenValidator;

use super::super::super::kernel::data::{AuthTicket, AuthTokenValue};
use super::super::data::DecodeAuthTokenError;

pub struct AuthJwtTokenValidator<'a> {
    key: &'a JwtTokenValidatorKey,
}

pub struct ApiJwtTokenValidator<'a> {
    key: &'a JwtTokenValidatorKey,
}

impl<'a> AuthJwtTokenValidator<'a> {
    pub const fn new(key: &'a JwtTokenValidatorKey) -> Self {
        Self { key }
    }
}

impl<'a> AuthTokenValidator for AuthJwtTokenValidator<'a> {
    fn validate(&self, token: &AuthTokenValue) -> Result<AuthTicket, DecodeAuthTokenError> {
        validate_jwt(token, &[AUTH_JWT_AUDIENCE_TICKET], &self.key.parse())
    }
}

impl<'a> ApiJwtTokenValidator<'a> {
    pub const fn with_key(key: &'a JwtTokenValidatorKey) -> Self {
        Self { key }
    }
}

impl<'a> AuthTokenValidator for ApiJwtTokenValidator<'a> {
    fn validate(&self, token: &AuthTokenValue) -> Result<AuthTicket, DecodeAuthTokenError> {
        validate_jwt(token, &[AUTH_JWT_AUDIENCE_API], &self.key.parse())
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
        decode::<AuthJwtClaims>(token.as_str(), &key, &validation).map_err(
            |err| match err.kind() {
                ErrorKind::ExpiredSignature => DecodeAuthTokenError::Expired,
                _ => DecodeAuthTokenError::Invalid(format!("{}", err)),
            },
        )?;

    Ok(data.claims.into_auth_ticket())
}

pub enum JwtTokenValidatorKey {
    Ec(String),
}

impl JwtTokenValidatorKey {
    fn parse<'a>(&'a self) -> DecodingKey<'a> {
        match self {
            Self::Ec(key) => {
                DecodingKey::from_ec_pem(key.as_bytes()).expect("failed to parse ec pem")
            }
        }
    }
}
