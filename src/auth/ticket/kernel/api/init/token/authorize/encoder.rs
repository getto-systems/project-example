use jsonwebtoken::{encode, EncodingKey};

use crate::auth::x_outside_feature::feature::AuthOutsideEncodingKey;

use crate::auth::ticket::kernel::init::token::authorize::data::Claims;

use crate::auth::ticket::encode::infra::AuthorizeTokenEncoder;

use crate::auth::{
    kernel::data::ExpireDateTime,
    ticket::{
        encode::data::EncodeAuthTokenError,
        kernel::data::{AuthTicket, AuthorizeToken},
    },
};

pub struct JwtAuthorizeTokenEncoder<'a> {
    key: &'a EncodingKey,
}

impl<'a> JwtAuthorizeTokenEncoder<'a> {
    pub fn new(encoding_key: &'a AuthOutsideEncodingKey) -> Self {
        Self {
            key: &encoding_key.authorize,
        }
    }
}

impl<'a> AuthorizeTokenEncoder for JwtAuthorizeTokenEncoder<'a> {
    fn encode(
        &self,
        ticket: AuthTicket,
        expires: ExpireDateTime,
    ) -> Result<(AuthorizeToken, ExpireDateTime), EncodeAuthTokenError> {
        let claims = Claims::new(ticket, expires.clone());

        let token = encode(&claims.header(), &claims, &self.key).map_err(|err| {
            EncodeAuthTokenError::InfraError(format!("encode jwt error; {}", err))
        })?;

        Ok((AuthorizeToken::restore(token), expires))
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::ticket::encode::infra::AuthorizeTokenEncoder;

    use crate::auth::{
        kernel::data::ExpireDateTime,
        ticket::{
            encode::data::EncodeAuthTokenError,
            kernel::data::{AuthTicket, AuthorizeToken},
        },
    };

    pub struct StaticAuthorizeTokenEncoder;

    impl AuthorizeTokenEncoder for StaticAuthorizeTokenEncoder {
        fn encode(
            &self,
            _ticket: AuthTicket,
            expires: ExpireDateTime,
        ) -> Result<(AuthorizeToken, ExpireDateTime), EncodeAuthTokenError> {
            Ok((AuthorizeToken::restore("TOKEN".to_owned()), expires))
        }
    }
}
