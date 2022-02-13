use std::collections::HashMap;

use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

use aws_cloudfront_cookie::CloudfrontPolicy;

use crate::auth::x_outside_feature::feature::AuthOutsideCloudfrontKey;

use crate::auth::ticket::{
    encode::infra::{AuthTokenEncoder, CloudfrontTokenEncoder},
    kernel::infra::AuthJwtClaims,
};

use crate::auth::ticket::{
    encode::data::EncodeAuthTokenError,
    kernel::data::{AuthTicket, AuthTokenExtract, CloudfrontTokenKind, ExpireDateTime},
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
    ) -> Result<AuthTokenExtract, EncodeAuthTokenError> {
        let (claims, expires) = AuthJwtClaims::new_ticket(ticket, expires);
        encode_jwt(JwtConfig {
            claims,
            expires,
            key: self.key,
        })
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
    ) -> Result<AuthTokenExtract, EncodeAuthTokenError> {
        let (claims, expires) = AuthJwtClaims::new_api(ticket, expires);
        encode_jwt(JwtConfig {
            claims,
            expires,
            key: self.key,
        })
    }
}

struct JwtConfig<'a> {
    claims: AuthJwtClaims,
    expires: i64,
    key: &'a EncodingKey,
}
fn encode_jwt<'a>(config: JwtConfig<'a>) -> Result<AuthTokenExtract, EncodeAuthTokenError> {
    let JwtConfig {
        claims,
        expires,
        key,
    } = config;

    let token = encode(&Header::new(Algorithm::ES384), &claims, key)
        .map_err(|err| EncodeAuthTokenError::InfraError(format!("{}", err)))?;

    Ok(AuthTokenExtract { token, expires })
}

pub struct CookieCloudfrontTokenEncoder<'a> {
    secret: &'a AuthOutsideCloudfrontKey,
}

impl<'a> CookieCloudfrontTokenEncoder<'a> {
    pub fn new(secret: &'a AuthOutsideCloudfrontKey) -> Self {
        Self { secret }
    }
}

impl<'a> CloudfrontTokenEncoder for CookieCloudfrontTokenEncoder<'a> {
    fn encode(
        &self,
        expires: ExpireDateTime,
    ) -> Result<HashMap<CloudfrontTokenKind, AuthTokenExtract>, EncodeAuthTokenError> {
        let policy = CloudfrontPolicy::from_resource(
            self.secret.resource.into(),
            expires.clone().extract().timestamp(),
        );
        let content = self
            .secret
            .key
            .sign(policy)
            .map_err(|err| EncodeAuthTokenError::InfraError(format!("sign error: {}", err)))?;

        let expires = expires.extract().timestamp();

        let mut map = HashMap::new();
        map.insert(
            CloudfrontTokenKind::KeyPairId,
            AuthTokenExtract {
                token: self.secret.key_pair_id.into(),
                expires,
            },
        );
        map.insert(
            CloudfrontTokenKind::Policy,
            AuthTokenExtract {
                token: content.policy,
                expires,
            },
        );
        map.insert(
            CloudfrontTokenKind::Signature,
            AuthTokenExtract {
                token: content.signature,
                expires,
            },
        );

        Ok(map)
    }
}

#[cfg(test)]
pub mod test {
    use std::collections::HashMap;

    use crate::auth::ticket::encode::infra::{AuthTokenEncoder, CloudfrontTokenEncoder};

    use crate::auth::ticket::{
        encode::data::EncodeAuthTokenError,
        kernel::data::{AuthTicket, AuthTokenExtract, CloudfrontTokenKind, ExpireDateTime},
    };

    pub struct StaticAuthTokenEncoder;

    impl AuthTokenEncoder for StaticAuthTokenEncoder {
        fn encode(
            &self,
            _ticket: AuthTicket,
            expires: ExpireDateTime,
        ) -> Result<AuthTokenExtract, EncodeAuthTokenError> {
            Ok(AuthTokenExtract {
                token: "TOKEN".into(),
                expires: expires.extract().timestamp(),
            })
        }
    }

    pub struct StaticCloudfrontTokenEncoder;

    impl CloudfrontTokenEncoder for StaticCloudfrontTokenEncoder {
        fn encode(
            &self,
            _expires: ExpireDateTime,
        ) -> Result<HashMap<CloudfrontTokenKind, AuthTokenExtract>, EncodeAuthTokenError> {
            Ok(HashMap::new())
        }
    }
}
