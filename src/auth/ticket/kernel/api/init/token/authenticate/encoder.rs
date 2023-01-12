use jsonwebtoken::{encode, EncodingKey};

use crate::auth::x_outside_feature::feature::AuthOutsideEncodingKey;

use crate::auth::ticket::kernel::init::token::authenticate::data::Claims;

use crate::auth::ticket::encode::infra::AuthenticateTokenEncoder;

use crate::auth::{
    kernel::data::ExpireDateTime,
    ticket::{
        encode::data::EncodeAuthTokenError,
        kernel::data::{AuthTicket, AuthenticateToken},
    },
};

pub struct JwtAuthenticateTokenEncoder<'a> {
    key: &'a EncodingKey,
}

impl<'a> JwtAuthenticateTokenEncoder<'a> {
    pub fn new(encoding_key: &'a AuthOutsideEncodingKey) -> Self {
        Self {
            key: &encoding_key.authenticate,
        }
    }
}

impl<'a> AuthenticateTokenEncoder for JwtAuthenticateTokenEncoder<'a> {
    fn encode(
        &self,
        ticket: AuthTicket,
        expires: ExpireDateTime,
    ) -> Result<(AuthenticateToken, ExpireDateTime), EncodeAuthTokenError> {
        let claims = Claims::new(ticket, expires.clone());

        let token = encode(&claims.header(), &claims, &self.key).map_err(|err| {
            EncodeAuthTokenError::InfraError(format!("encode jwt error; {}", err))
        })?;

        Ok((AuthenticateToken::restore(token), expires))
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::ticket::encode::infra::AuthenticateTokenEncoder;

    use crate::auth::{
        kernel::data::ExpireDateTime,
        ticket::{
            encode::data::EncodeAuthTokenError,
            kernel::data::{AuthTicket, AuthenticateToken},
        },
    };

    pub struct StaticAuthenticateTokenEncoder;

    impl AuthenticateTokenEncoder for StaticAuthenticateTokenEncoder {
        fn encode(
            &self,
            _ticket: AuthTicket,
            expires: ExpireDateTime,
        ) -> Result<(AuthenticateToken, ExpireDateTime), EncodeAuthTokenError> {
            Ok((AuthenticateToken::restore("TOKEN".to_owned()), expires))
        }
    }
}
