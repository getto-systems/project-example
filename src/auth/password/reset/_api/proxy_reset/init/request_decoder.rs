use prost::Message;

use crate::auth::password::reset::_api::y_protobuf::api::ResetPasswordPb;

use crate::z_details::_api::message::helper::{decode_base64, invalid_protobuf};

use crate::auth::password::reset::{
    _api::proxy_reset::infra::ResetPasswordProxyRequestDecoder,
    _common::reset::infra::ResetPasswordFieldsExtract,
};

use crate::z_details::_api::message::data::MessageError;

pub struct RequestDecoder {
    body: String,
}

impl RequestDecoder {
    pub const fn new(body: String) -> Self {
        Self { body }
    }
}

impl ResetPasswordProxyRequestDecoder for RequestDecoder {
    fn decode(self) -> Result<ResetPasswordFieldsExtract, MessageError> {
        let message =
            ResetPasswordPb::decode(decode_base64(self.body)?).map_err(invalid_protobuf)?;

        Ok(ResetPasswordFieldsExtract {
            reset_token: message.reset_token,
            login_id: message.login_id,
            password: message.password,
        })
    }
}
