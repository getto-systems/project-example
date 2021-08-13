use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

use crate::auth::_auth::x_outside_feature::feature::AuthOutsideSecret;

use crate::auth::password::reset::_auth::{
    kernel::infra::ResetTokenJwtClaims, request_token::infra::ResetTokenEncoder,
};

use crate::auth::{
    auth_ticket::_common::kernel::data::ExpireDateTime,
    password::{
        _auth::kernel::data::ResetToken,
        reset::_auth::{
            kernel::data::ResetTokenEncoded, request_token::data::EncodeResetTokenError,
        },
    },
};

pub struct JwtResetTokenEncoder<'a> {
    key: &'a EncodingKey,
}

impl<'a> JwtResetTokenEncoder<'a> {
    pub const fn new(secret: &'a AuthOutsideSecret) -> Self {
        Self {
            key: &secret.reset_token.encoding_key,
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
        auth_ticket::_common::kernel::data::ExpireDateTime,
        password::{
            _auth::kernel::data::ResetToken, reset::_auth::kernel::data::ResetTokenEncoded,
        },
    };

    pub struct StaticResetTokenEncoder;

    impl<'a> StaticResetTokenEncoder {
        pub fn new() -> Self {
            Self
        }
    }

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
