use std::sync::Arc;

use jsonwebtoken::{encode, EncodingKey};

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::{
    auth::user::password::reset::kernel::detail::token::data::Claims, common::api::feature::AsInfra,
};

use crate::auth::user::password::reset::request_token::infra::ResetPasswordTokenEncoder;

use crate::auth::{
    kernel::data::ExpireDateTime,
    user::password::reset::{
        kernel::data::{ResetPasswordId, ResetPasswordToken},
        request_token::data::EncodeResetTokenError,
    },
};

pub struct JwtResetPasswordTokenEncoder {
    key: Arc<EncodingKey>,
}

impl AsInfra<JwtResetPasswordTokenEncoder> for Arc<AuthAppFeature> {
    fn as_infra(&self) -> JwtResetPasswordTokenEncoder {
        JwtResetPasswordTokenEncoder {
            key: Arc::clone(&self.reset_token_key.encoding_key),
        }
    }
}

impl ResetPasswordTokenEncoder for JwtResetPasswordTokenEncoder {
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

    pub struct MockResetTokenEncoder;

    impl<'a> ResetPasswordTokenEncoder for MockResetTokenEncoder {
        fn encode(
            &self,
            _token: ResetPasswordId,
            _expires: ExpireDateTime,
        ) -> Result<ResetPasswordToken, EncodeResetTokenError> {
            Ok(ResetPasswordToken::restore("TOKEN".to_owned()))
        }
    }
}
