use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

use aws_cloudfront_cookie::{CloudfrontKey, CloudfrontPolicy};

use crate::auth::_api::x_outside_feature::feature::{AuthOutsideCdnSecret, AuthOutsideCookie};

use super::super::super::kernel::x_actix_web::header::{
    COOKIE_API_TOKEN, COOKIE_CLOUDFRONT_KEY_PAIR_ID, COOKIE_CLOUDFRONT_POLICY,
    COOKIE_CLOUDFRONT_SIGNATURE, COOKIE_TICKET_TOKEN,
};

use super::AuthTokenEncoder;
use crate::auth::auth_ticket::_api::kernel::infra::{
    AUTH_JWT_AUDIENCE_API, AUTH_JWT_AUDIENCE_TICKET,
};

use super::super::super::kernel::data::{AuthTicket, AuthTokenExtract, ExpireDateTime};
use super::super::data::{AuthTokenEncoded, EncodeAuthTokenError};

pub struct TicketJwtTokenEncoder<'a> {
    domain: &'a str,
    key: &'a EncodingKey,
}

impl<'a> TicketJwtTokenEncoder<'a> {
    pub fn new(cookie: &'a AuthOutsideCookie, key: &'a EncodingKey) -> Self {
        Self {
            domain: &cookie.domain,
            key,
        }
    }
}

impl<'a> AuthTokenEncoder for TicketJwtTokenEncoder<'a> {
    fn encode(
        &self,
        ticket: AuthTicket,
        expires: ExpireDateTime,
    ) -> Result<Vec<AuthTokenEncoded>, EncodeAuthTokenError> {
        Ok(vec![encode_jwt(JwtConfig {
            domain: self.domain,
            name: COOKIE_TICKET_TOKEN,
            aud: AUTH_JWT_AUDIENCE_TICKET,
            ticket,
            expires,
            key: self.key,
        })?])
    }
}

pub struct ApiJwtTokenEncoder<'a> {
    domain: &'a str,
    key: &'a EncodingKey,
}

impl<'a> ApiJwtTokenEncoder<'a> {
    pub fn new(cookie: &'a AuthOutsideCookie, key: &'a EncodingKey) -> Self {
        Self {
            domain: &cookie.domain,
            key,
        }
    }
}

impl<'a> AuthTokenEncoder for ApiJwtTokenEncoder<'a> {
    fn encode(
        &self,
        ticket: AuthTicket,
        expires: ExpireDateTime,
    ) -> Result<Vec<AuthTokenEncoded>, EncodeAuthTokenError> {
        Ok(vec![encode_jwt(JwtConfig {
            domain: self.domain,
            name: COOKIE_API_TOKEN,
            aud: AUTH_JWT_AUDIENCE_API,
            ticket,
            expires,
            key: self.key,
        })?])
    }
}

struct JwtConfig<'a> {
    domain: &'a str,
    name: &'static str,
    aud: &'static str,
    ticket: AuthTicket,
    expires: ExpireDateTime,
    key: &'a EncodingKey,
}
fn encode_jwt<'a>(config: JwtConfig<'a>) -> Result<AuthTokenEncoded, EncodeAuthTokenError> {
    let JwtConfig {
        domain,
        name,
        aud,
        ticket,
        expires,
        key,
    } = config;

    let token = encode(
        &Header::new(Algorithm::ES384),
        &ticket.into_jwt_claims(aud.into(), expires.clone()),
        key,
    )
    .map_err(|err| EncodeAuthTokenError::InfraError(format!("{}", err)))?;

    Ok(AuthTokenEncoded {
        domain: domain.into(),
        name: name.into(),
        token: AuthTokenExtract {
            value: token,
            expires,
        },
    })
}

pub enum JwtTokenEncoderKey {}

impl JwtTokenEncoderKey {
    pub fn ec(key: String) -> EncodingKey {
        EncodingKey::from_ec_pem(key.as_bytes()).expect("failed to parse ec pem")
    }
}

pub struct CloudfrontTokenEncoder<'a> {
    domain: &'a str,
    key: &'a CloudfrontKey,
    key_pair_id: &'a str,
    resource: &'a str,
}

impl<'a> CloudfrontTokenEncoder<'a> {
    pub fn new(secret: &'a AuthOutsideCdnSecret, cookie: &'a AuthOutsideCookie) -> Self {
        Self {
            domain: &cookie.domain,
            key: &secret.key,
            key_pair_id: &cookie.cloudfront_key_pair_id,
            resource: &cookie.cloudfront_resource,
        }
    }
}

impl<'a> AuthTokenEncoder for CloudfrontTokenEncoder<'a> {
    fn encode(
        &self,
        _ticket: AuthTicket,
        expires: ExpireDateTime,
    ) -> Result<Vec<AuthTokenEncoded>, EncodeAuthTokenError> {
        let policy = CloudfrontPolicy::from_resource(self.resource.into(), expires.timestamp());
        let content = self
            .key
            .sign_sha1(policy)
            .map_err(|err| EncodeAuthTokenError::InfraError(format!("sign error: {}", err)))?;

        Ok(vec![
            AuthTokenEncoded {
                domain: self.domain.into(),
                name: COOKIE_CLOUDFRONT_KEY_PAIR_ID.into(),
                token: AuthTokenExtract {
                    value: self.key_pair_id.into(),
                    expires: expires.clone(),
                },
            },
            AuthTokenEncoded {
                domain: self.domain.into(),
                name: COOKIE_CLOUDFRONT_POLICY.into(),
                token: AuthTokenExtract {
                    value: content.policy,
                    expires: expires.clone(),
                },
            },
            AuthTokenEncoded {
                domain: self.domain.into(),
                name: COOKIE_CLOUDFRONT_SIGNATURE.into(),
                token: AuthTokenExtract {
                    value: content.signature,
                    expires: expires.clone(),
                },
            },
        ])
    }
}
