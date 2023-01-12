use jsonwebtoken::{encode, EncodingKey};

use crate::auth::x_outside_feature::feature::AuthOutsideResetTokenKey;

use crate::auth::user::password::reset::request_token::infra::ResetPasswordTokenEncoder;

use crate::auth::{
    kernel::data::ExpireDateTime,
    user::password::reset::{
        kernel::{
            data::{ResetPasswordId, ResetPasswordToken},
            init::token::data::Claims,
        },
        request_token::data::EncodeResetTokenError,
    },
};

pub struct JwtResetPasswordTokenEncoder<'a> {
    key: &'a EncodingKey,
}

impl<'a> JwtResetPasswordTokenEncoder<'a> {
    pub const fn new(key: &'a AuthOutsideResetTokenKey) -> Self {
        Self {
            key: &key.encoding_key,
        }
    }
}

impl<'a> ResetPasswordTokenEncoder for JwtResetPasswordTokenEncoder<'a> {
    fn encode(
        &self,
        token: ResetPasswordId,
        expires: ExpireDateTime,
    ) -> Result<ResetPasswordToken, EncodeResetTokenError> {
        let claims = Claims::new(token, expires);

        let token = encode(&claims.header(), &claims, &self.key)
            .map_err(|err| EncodeResetTokenError::InfraError(format!("{}", err)))?;

        Ok(ResetPasswordToken::restore(token))
    }
}

#[cfg(test)]
pub mod test {
    use super::ResetPasswordTokenEncoder;

    use crate::auth::{
        kernel::data::ExpireDateTime,
        user::password::reset::{
            kernel::data::{ResetPasswordId, ResetPasswordToken},
            request_token::data::EncodeResetTokenError,
        },
    };

    pub struct StaticResetTokenEncoder;

    impl<'a> ResetPasswordTokenEncoder for StaticResetTokenEncoder {
        fn encode(
            &self,
            _token: ResetPasswordId,
            _expires: ExpireDateTime,
        ) -> Result<ResetPasswordToken, EncodeResetTokenError> {
            Ok(ResetPasswordToken::restore("TOKEN".to_owned()))
        }
    }
}
