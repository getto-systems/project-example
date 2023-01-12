use jsonwebtoken::{decode, errors::ErrorKind, DecodingKey};

use crate::auth::x_outside_feature::feature::AuthOutsideResetTokenKey;

use crate::auth::user::password::reset::reset::infra::ResetPasswordTokenDecoder;

use crate::auth::user::password::reset::{
    kernel::{
        data::{ResetPasswordId, ResetPasswordToken},
        init::token::data::Claims,
    },
    reset::data::DecodeResetTokenError,
};

pub struct JwtResetPasswordTokenDecoder<'a> {
    key: &'a DecodingKey,
}

impl<'a> JwtResetPasswordTokenDecoder<'a> {
    pub const fn new(key: &'a AuthOutsideResetTokenKey) -> Self {
        Self {
            key: &key.decoding_key,
        }
    }
}

impl<'a> ResetPasswordTokenDecoder for JwtResetPasswordTokenDecoder<'a> {
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
    use crate::auth::user::password::reset::reset::infra::ResetPasswordTokenDecoder;

    use crate::auth::user::password::reset::{
        kernel::data::{ResetPasswordId, ResetPasswordToken},
        reset::data::DecodeResetTokenError,
    };

    pub enum StaticResetTokenDecoder {
        Valid(ResetPasswordId),
        Expired,
    }

    impl ResetPasswordTokenDecoder for StaticResetTokenDecoder {
        fn decode(
            &self,
            _token: ResetPasswordToken,
        ) -> Result<ResetPasswordId, DecodeResetTokenError> {
            match self {
                Self::Valid(token) => Ok(token.clone()),
                Self::Expired => Err(DecodeResetTokenError::Expired),
            }
        }
    }
}
