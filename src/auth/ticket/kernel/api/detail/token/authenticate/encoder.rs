use std::sync::Arc;

use jsonwebtoken::{encode, EncodingKey};

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::auth::ticket::kernel::detail::token::authenticate::data::Claims;

use crate::auth::ticket::encode::infra::AuthenticateTokenEncoder;

use crate::auth::{
    kernel::data::ExpireDateTime,
    ticket::{
        encode::data::EncodeTokenError,
        kernel::data::{AuthTicket, AuthenticateToken},
    },
};
use crate::common::api::feature::AsInfra;

pub struct JwtAuthenticateTokenEncoder {
    key: Arc<EncodingKey>,
}

impl AsInfra<JwtAuthenticateTokenEncoder> for Arc<AuthAppFeature> {
    fn as_infra(&self) -> JwtAuthenticateTokenEncoder {
        JwtAuthenticateTokenEncoder {
            key: Arc::clone(&self.encoding_key.authenticate),
        }
    }
}

impl AuthenticateTokenEncoder for JwtAuthenticateTokenEncoder {
    fn encode(
        &self,
        ticket: AuthTicket,
        expires: ExpireDateTime,
    ) -> Result<(AuthenticateToken, ExpireDateTime), EncodeTokenError> {
        let claims = Claims::new(ticket, expires.clone());

        let token = encode(&claims.header(), &claims, &self.key)
            .map_err(|err| EncodeTokenError::InfraError(format!("encode jwt error; {}", err)))?;

        Ok((AuthenticateToken::restore(token), expires))
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::ticket::encode::infra::AuthenticateTokenEncoder;

    use crate::auth::{
        kernel::data::ExpireDateTime,
        ticket::{
            encode::data::EncodeTokenError,
            kernel::data::{AuthTicket, AuthenticateToken},
        },
    };

    pub struct StaticAuthenticateTokenEncoder;

    impl AuthenticateTokenEncoder for StaticAuthenticateTokenEncoder {
        fn encode(
            &self,
            _ticket: AuthTicket,
            expires: ExpireDateTime,
        ) -> Result<(AuthenticateToken, ExpireDateTime), EncodeTokenError> {
            Ok((AuthenticateToken::restore("TOKEN".to_owned()), expires))
        }
    }
}
