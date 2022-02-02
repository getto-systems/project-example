use jsonwebtoken::{decode, errors::ErrorKind, Algorithm, DecodingKey, Validation};

use crate::auth::remote::x_outside_feature::auth::feature::AuthOutsideResetTokenKey;

use crate::auth::user::password::reset::{
    kernel::infra::ResetTokenJwtClaims, reset::remote::infra::ResetTokenDecoder,
};

use crate::auth::user::password::reset::{
    kernel::data::{ResetToken, ResetTokenEncoded},
    reset::remote::data::DecodeResetTokenError,
};

pub struct JwtResetTokenDecoder<'a> {
    key: &'a DecodingKey<'a>,
}

impl<'a> JwtResetTokenDecoder<'a> {
    pub const fn new(key: &'a AuthOutsideResetTokenKey) -> Self {
        Self {
            key: &key.decoding_key,
        }
    }
}

impl<'a> ResetTokenDecoder for JwtResetTokenDecoder<'a> {
    fn decode(&self, token: &ResetTokenEncoded) -> Result<ResetToken, DecodeResetTokenError> {
        validate_jwt(token, &self.key)
    }
}

fn validate_jwt<'a>(
    token: &ResetTokenEncoded,
    key: &DecodingKey<'a>,
) -> Result<ResetToken, DecodeResetTokenError> {
    let validation = Validation::new(Algorithm::ES384);

    let data =
        decode::<ResetTokenJwtClaims>(token.as_str(), &key, &validation).map_err(
            |err| match err.kind() {
                ErrorKind::ExpiredSignature => DecodeResetTokenError::Expired,
                _ => DecodeResetTokenError::Invalid(format!("{}", err)),
            },
        )?;

    Ok(data.claims.into())
}

#[cfg(test)]
pub mod test {
    use crate::auth::user::password::reset::reset::remote::infra::ResetTokenDecoder;

    use crate::auth::user::password::reset::{
        kernel::data::{ResetToken, ResetTokenEncoded},
        reset::remote::data::DecodeResetTokenError,
    };

    pub enum StaticResetTokenDecoder {
        Valid(ResetToken),
        Expired,
    }

    impl ResetTokenDecoder for StaticResetTokenDecoder {
        fn decode(&self, _token: &ResetTokenEncoded) -> Result<ResetToken, DecodeResetTokenError> {
            match self {
                Self::Expired => Err(DecodeResetTokenError::Expired),
                Self::Valid(token) => Ok(token.clone()),
            }
        }
    }
}
