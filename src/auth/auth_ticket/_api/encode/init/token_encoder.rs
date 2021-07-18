use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

use aws_cloudfront_cookie::{CloudfrontKey, CloudfrontPolicy};

use crate::auth::_api::x_outside_feature::feature::{AuthOutsideCloudfrontSecret, AuthOutsideCookie};

use crate::auth::auth_ticket::_api::kernel::x_actix_web::header::{
    COOKIE_API_TOKEN, COOKIE_CLOUDFRONT_KEY_PAIR_ID, COOKIE_CLOUDFRONT_POLICY,
    COOKIE_CLOUDFRONT_SIGNATURE, COOKIE_TICKET_TOKEN,
};

use crate::auth::auth_ticket::_api::{
    encode::infra::AuthTokenEncoder,
    kernel::infra::{AuthJwtClaims, AUTH_JWT_AUDIENCE_API, AUTH_JWT_AUDIENCE_TICKET},
};

use super::super::super::kernel::data::{AuthTicket, AuthTokenExtract, ExpireDateTime};
use super::super::data::{AuthTokenEncodedData, EncodeAuthTokenError};

pub struct TicketJwtAuthTokenEncoder<'a> {
    domain: &'a str,
    key: &'a EncodingKey,
}

impl<'a> TicketJwtAuthTokenEncoder<'a> {
    pub fn new(cookie: &'a AuthOutsideCookie, key: &'a EncodingKey) -> Self {
        Self {
            domain: &cookie.domain,
            key,
        }
    }
}

impl<'a> AuthTokenEncoder for TicketJwtAuthTokenEncoder<'a> {
    fn encode(
        &self,
        ticket: AuthTicket,
        expires: ExpireDateTime,
    ) -> Result<Vec<AuthTokenEncodedData>, EncodeAuthTokenError> {
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

pub struct ApiJwtAuthTokenEncoder<'a> {
    domain: &'a str,
    key: &'a EncodingKey,
}

impl<'a> ApiJwtAuthTokenEncoder<'a> {
    pub fn new(cookie: &'a AuthOutsideCookie, key: &'a EncodingKey) -> Self {
        Self {
            domain: &cookie.domain,
            key,
        }
    }
}

impl<'a> AuthTokenEncoder for ApiJwtAuthTokenEncoder<'a> {
    fn encode(
        &self,
        ticket: AuthTicket,
        expires: ExpireDateTime,
    ) -> Result<Vec<AuthTokenEncodedData>, EncodeAuthTokenError> {
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
fn encode_jwt<'a>(config: JwtConfig<'a>) -> Result<AuthTokenEncodedData, EncodeAuthTokenError> {
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
        &AuthJwtClaims::from_ticket(ticket, aud.into(), expires.clone()),
        key,
    )
    .map_err(|err| EncodeAuthTokenError::InfraError(format!("{}", err)))?;

    Ok(AuthTokenEncodedData {
        domain: domain.into(),
        name: name.into(),
        token: AuthTokenExtract {
            value: token,
            expires,
        },
    })
}

pub struct CloudfrontTokenEncoder<'a> {
    domain: &'a str,
    key: &'a CloudfrontKey,
    key_pair_id: &'a str,
    resource: &'a str,
}

impl<'a> CloudfrontTokenEncoder<'a> {
    pub fn new(secret: &'a AuthOutsideCloudfrontSecret, cookie: &'a AuthOutsideCookie) -> Self {
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
    ) -> Result<Vec<AuthTokenEncodedData>, EncodeAuthTokenError> {
        let policy = CloudfrontPolicy::from_resource(
            self.resource.into(),
            expires.clone().extract().timestamp(),
        );
        let content = self
            .key
            .sign(policy)
            .map_err(|err| EncodeAuthTokenError::InfraError(format!("sign error: {}", err)))?;

        Ok(vec![
            AuthTokenEncodedData {
                domain: self.domain.into(),
                name: COOKIE_CLOUDFRONT_KEY_PAIR_ID.into(),
                token: AuthTokenExtract {
                    value: self.key_pair_id.into(),
                    expires: expires.clone(),
                },
            },
            AuthTokenEncodedData {
                domain: self.domain.into(),
                name: COOKIE_CLOUDFRONT_POLICY.into(),
                token: AuthTokenExtract {
                    value: content.policy,
                    expires: expires.clone(),
                },
            },
            AuthTokenEncodedData {
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

#[cfg(test)]
pub mod test {
    use super::AuthTokenEncoder;

    use super::super::super::super::kernel::data::{AuthTicket, ExpireDateTime};
    use super::super::super::data::{AuthTokenEncodedData, EncodeAuthTokenError};

    pub struct StaticAuthTokenEncoder;

    impl<'a> StaticAuthTokenEncoder {
        pub fn new() -> Self {
            Self
        }
    }

    impl<'a> AuthTokenEncoder for StaticAuthTokenEncoder {
        fn encode(
            &self,
            _ticket: AuthTicket,
            _expires: ExpireDateTime,
        ) -> Result<Vec<AuthTokenEncodedData>, EncodeAuthTokenError> {
            Ok(vec![])
        }
    }
}
