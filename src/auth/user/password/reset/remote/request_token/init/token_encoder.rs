use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

use crate::auth::_auth::x_outside_feature::feature::AuthOutsideResetTokenKey;

use crate::auth::user::password::reset::remote::{
    kernel::infra::ResetTokenJwtClaims, request_token::infra::ResetTokenEncoder,
};

use crate::auth::{
    ticket::remote::kernel::data::ExpireDateTime,
    user::password::{
        remote::kernel::data::ResetToken,
        reset::remote::{
            kernel::data::ResetTokenEncoded, request_token::data::EncodeResetTokenError,
        },
    },
};

pub struct JwtResetTokenEncoder<'a> {
    key: &'a EncodingKey,
}

impl<'a> JwtResetTokenEncoder<'a> {
    pub const fn new(key: &'a AuthOutsideResetTokenKey) -> Self {
        Self {
            key: &key.encoding_key,
        }
    }
}

impl<'a> ResetTokenEncoder for JwtResetTokenEncoder<'a> {
    fn encode(
        &self,
        token: ResetToken,
        expires: ExpireDateTime,
    ) -> Result<ResetTokenEncoded, EncodeResetTokenError> {
        Ok(encode_jwt(JwtConfig {
            token,
            expires,
            key: self.key,
        })?)
    }
}

struct JwtConfig<'a> {
    token: ResetToken,
    expires: ExpireDateTime,
    key: &'a EncodingKey,
}
fn encode_jwt<'a>(config: JwtConfig<'a>) -> Result<ResetTokenEncoded, EncodeResetTokenError> {
    let JwtConfig {
        token,
        expires,
        key,
    } = config;

    let token = encode(
        &Header::new(Algorithm::ES384),
        &ResetTokenJwtClaims::from_token(token, expires),
        key,
    )
    .map_err(|err| EncodeResetTokenError::InfraError(format!("{}", err)))?;

    Ok(ResetTokenEncoded::new(token))
}

#[cfg(test)]
pub mod test {
    use super::ResetTokenEncoder;

    use super::super::super::data::EncodeResetTokenError;
    use crate::auth::{
        ticket::remote::kernel::data::ExpireDateTime,
        user::password::{
            remote::kernel::data::ResetToken, reset::remote::kernel::data::ResetTokenEncoded,
        },
    };

    pub struct StaticResetTokenEncoder;

    impl<'a> ResetTokenEncoder for StaticResetTokenEncoder {
        fn encode(
            &self,
            _token: ResetToken,
            _expires: ExpireDateTime,
        ) -> Result<ResetTokenEncoded, EncodeResetTokenError> {
            Ok(ResetTokenEncoded::new("encoded".into()))
        }
    }
}
