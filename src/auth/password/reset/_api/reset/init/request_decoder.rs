use prost::Message;

use crate::auth::password::reset::_api::y_protobuf::api::ResetPasswordPb;

use crate::z_details::_api::message::helper::{decode_base64, invalid_protobuf};

use crate::auth::password::reset::_api::reset::infra::{
    ResetPasswordFieldsExtract, ResetPasswordRequestDecoder,
};

use crate::z_details::_api::message::data::MessageError;

pub struct ProstResetPasswordRequestDecoder {
    body: String,
}

impl ProstResetPasswordRequestDecoder {
    pub const fn new(body: String) -> Self {
        Self { body }
    }
}

impl ResetPasswordRequestDecoder for ProstResetPasswordRequestDecoder {
    fn decode(&self) -> Result<ResetPasswordFieldsExtract, MessageError> {
        // TODO body は clone したくない
        let message =
            ResetPasswordPb::decode(decode_base64(self.body.clone())?).map_err(invalid_protobuf)?;

        Ok(ResetPasswordFieldsExtract {
            reset_token: message.reset_token,
            login_id: message.login_id,
            password: message.password,
        })
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::password::reset::_api::reset::infra::{
        ResetPasswordFieldsExtract, ResetPasswordRequestDecoder,
    };

    use crate::z_details::_api::message::data::MessageError;

    pub struct StaticResetPasswordRequestDecoder {
        pub fields: ResetPasswordFieldsExtract,
    }

    impl ResetPasswordRequestDecoder for StaticResetPasswordRequestDecoder {
        fn decode(&self) -> Result<ResetPasswordFieldsExtract, MessageError> {
            // TODO self を consume する api にすれば clone しなくて良くなる
            Ok(self.fields.clone())
        }
    }
}
