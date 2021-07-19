use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

use aws_cloudfront_cookie::CloudfrontPolicy;

use crate::auth::_auth::x_outside_feature::feature::AuthOutsideCloudfrontSecret;

use crate::auth::auth_ticket::_auth::encode::data::AuthTokenKind;
use crate::auth::auth_ticket::_auth::{
    encode::infra::AuthTokenEncoder,
    kernel::infra::{AuthJwtClaims, AUTH_JWT_AUDIENCE_API, AUTH_JWT_AUDIENCE_TICKET},
};

use crate::auth::auth_ticket::_auth::{
    encode::data::{AuthTokenEncodedData, EncodeAuthTokenError},
    kernel::data::{AuthTicket, AuthTokenExtract, ExpireDateTime},
};

pub struct TicketJwtAuthTokenEncoder<'a> {
    key: &'a EncodingKey,
}

impl<'a> TicketJwtAuthTokenEncoder<'a> {
    pub fn new(key: &'a EncodingKey) -> Self {
        Self { key }
    }
}

impl<'a> AuthTokenEncoder for TicketJwtAuthTokenEncoder<'a> {
    fn encode(
        &self,
        ticket: AuthTicket,
        expires: ExpireDateTime,
    ) -> Result<Vec<AuthTokenEncodedData>, EncodeAuthTokenError> {
        Ok(vec![encode_jwt(JwtConfig {
            kind: AuthTokenKind::Ticket,
            aud: AUTH_JWT_AUDIENCE_TICKET,
            ticket,
            expires,
            key: self.key,
        })?])
    }
}

pub struct ApiJwtAuthTokenEncoder<'a> {
    key: &'a EncodingKey,
}

impl<'a> ApiJwtAuthTokenEncoder<'a> {
    pub fn new(key: &'a EncodingKey) -> Self {
        Self { key }
    }
}

impl<'a> AuthTokenEncoder for ApiJwtAuthTokenEncoder<'a> {
    fn encode(
        &self,
        ticket: AuthTicket,
        expires: ExpireDateTime,
    ) -> Result<Vec<AuthTokenEncodedData>, EncodeAuthTokenError> {
        Ok(vec![encode_jwt(JwtConfig {
            kind: AuthTokenKind::Api,
            aud: AUTH_JWT_AUDIENCE_API,
            ticket,
            expires,
            key: self.key,
        })?])
    }
}

struct JwtConfig<'a> {
    kind: AuthTokenKind,
    aud: &'static str,
    ticket: AuthTicket,
    expires: ExpireDateTime,
    key: &'a EncodingKey,
}
fn encode_jwt<'a>(config: JwtConfig<'a>) -> Result<AuthTokenEncodedData, EncodeAuthTokenError> {
    let JwtConfig {
        kind,
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
        kind,
        token: AuthTokenExtract {
            value: token,
            expires,
        },
    })
}

pub struct CloudfrontTokenEncoder<'a> {
    secret: &'a AuthOutsideCloudfrontSecret,
}

impl<'a> CloudfrontTokenEncoder<'a> {
    pub fn new(secret: &'a AuthOutsideCloudfrontSecret) -> Self {
        Self { secret }
    }
}

impl<'a> AuthTokenEncoder for CloudfrontTokenEncoder<'a> {
    fn encode(
        &self,
        _ticket: AuthTicket,
        expires: ExpireDateTime,
    ) -> Result<Vec<AuthTokenEncodedData>, EncodeAuthTokenError> {
        let policy = CloudfrontPolicy::from_resource(
            self.secret.resource.into(),
            expires.clone().extract().timestamp(),
        );
        let content = self
            .secret
            .key
            .sign(policy)
            .map_err(|err| EncodeAuthTokenError::InfraError(format!("sign error: {}", err)))?;

        Ok(vec![
            AuthTokenEncodedData {
                kind: AuthTokenKind::CloudfrontKeyPairId,
                token: AuthTokenExtract {
                    value: self.secret.key_pair_id.into(),
                    expires: expires.clone(),
                },
            },
            AuthTokenEncodedData {
                kind: AuthTokenKind::CloudfrontPolicy,
                token: AuthTokenExtract {
                    value: content.policy,
                    expires: expires.clone(),
                },
            },
            AuthTokenEncodedData {
                kind: AuthTokenKind::CloudfrontSignature,
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

    use crate::auth::auth_ticket::_auth::{
        encode::data::{AuthTokenEncodedData, EncodeAuthTokenError},
        kernel::data::{AuthTicket, ExpireDateTime},
    };

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
