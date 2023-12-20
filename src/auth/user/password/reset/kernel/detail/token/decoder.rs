use std::sync::Arc;

use jsonwebtoken::{decode, errors::ErrorKind, DecodingKey};

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::common::api::feature::AsInfra;

use crate::auth::user::password::reset::reset::infra::ResetPasswordTokenDecoder;

use crate::auth::user::password::reset::{
    kernel::{
        data::{ResetPasswordId, ResetPasswordToken},
        detail::token::data::Claims,
    },
    reset::data::DecodeResetTokenError,
};

pub struct JwtResetPasswordTokenDecoder {
    key: Arc<DecodingKey>,
}

impl AsInfra<JwtResetPasswordTokenDecoder> for Arc<AuthAppFeature> {
    fn as_infra(&self) -> JwtResetPasswordTokenDecoder {
        JwtResetPasswordTokenDecoder {
            key: Arc::clone(&self.reset_token_key.decoding_key),
        }
    }
}

impl ResetPasswordTokenDecoder for JwtResetPasswordTokenDecoder {
    fn decode(&self, token: ResetPasswordToken) -> Result<ResetPasswordId, DecodeResetTokenError> {
        let data = decode::<Claims>(&token.extract(), &self.key, &Claims::validation()).map_err(
            |err| match err.kind() {
                ErrorKind::ExpiredSignature => DecodeResetTokenError::Expired,
                _ => DecodeResetTokenError::Invalid(format!("{}", err)),
            },
        )?;

        Ok(data.claims.into())
    }
}

#[cfg(test)]
pub mod test {
    use std::collections::HashMap;

    use crate::auth::user::password::reset::reset::infra::ResetPasswordTokenDecoder;

    use crate::auth::user::password::reset::{
        kernel::data::{ResetPasswordId, ResetPasswordToken},
        reset::data::DecodeResetTokenError,
    };

    #[derive(Clone)]
    pub struct MockResetTokenDecoder(
        HashMap<String, Result<ResetPasswordId, DecodeResetTokenError>>,
    );

    impl MockResetTokenDecoder {
        pub fn new(map: Vec<(String, Result<ResetPasswordId, DecodeResetTokenError>)>) -> Self {
            Self(map.into_iter().collect())
        }
    }

    impl ResetPasswordTokenDecoder for MockResetTokenDecoder {
        fn decode(
            &self,
            token: ResetPasswordToken,
        ) -> Result<ResetPasswordId, DecodeResetTokenError> {
            match self.0.get(&token.extract()) {
                Some(result) => result.clone(),
                None => Err(DecodeResetTokenError::Invalid(
                    "invalid authenticate token".to_owned(),
                )),
            }
        }
    }
}
