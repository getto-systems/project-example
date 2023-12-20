use std::sync::Arc;

use jsonwebtoken::{encode, EncodingKey};

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::{
    auth::ticket::kernel::detail::token::authorize::data::Claims, common::api::feature::AsInfra,
};

use crate::auth::ticket::encode::infra::AuthorizeTokenEncoder;

use crate::auth::{
    kernel::data::ExpireDateTime,
    ticket::{
        encode::data::EncodeTokenError,
        kernel::data::{AuthTicket, AuthorizeToken},
    },
};

pub struct JwtAuthorizeTokenEncoder {
    key: Arc<EncodingKey>,
}

impl AsInfra<JwtAuthorizeTokenEncoder> for Arc<AuthAppFeature> {
    fn as_infra(&self) -> JwtAuthorizeTokenEncoder {
        JwtAuthorizeTokenEncoder {
            key: Arc::clone(&self.encoding_key.authorize),
        }
    }
}

impl AuthorizeTokenEncoder for JwtAuthorizeTokenEncoder {
    fn encode(
        &self,
        ticket: AuthTicket,
        expires: ExpireDateTime,
    ) -> Result<(AuthorizeToken, ExpireDateTime), EncodeTokenError> {
        let claims = Claims::new(ticket, expires.clone());

        let token = encode(&claims.header(), &claims, &self.key)
            .map_err(|err| EncodeTokenError::InfraError(format!("encode jwt error; {}", err)))?;

        Ok((AuthorizeToken::restore(token), expires))
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::ticket::encode::infra::AuthorizeTokenEncoder;

    use crate::auth::{
        kernel::data::ExpireDateTime,
        ticket::{
            encode::data::EncodeTokenError,
            kernel::data::{AuthTicket, AuthorizeToken},
        },
    };

    pub struct StaticAuthorizeTokenEncoder;

    impl AuthorizeTokenEncoder for StaticAuthorizeTokenEncoder {
        fn encode(
            &self,
            _ticket: AuthTicket,
            expires: ExpireDateTime,
        ) -> Result<(AuthorizeToken, ExpireDateTime), EncodeTokenError> {
            Ok((AuthorizeToken::restore("TOKEN".to_owned()), expires))
        }
    }
}
