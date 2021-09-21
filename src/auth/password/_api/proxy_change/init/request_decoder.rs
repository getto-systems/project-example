use prost::Message;

use crate::auth::password::_api::y_protobuf::api::ChangePasswordPb;

use crate::auth::password::_common::change::infra::ChangePasswordFieldsExtract;
use crate::z_details::_api::message::helper::{decode_base64, invalid_protobuf};

use crate::auth::password::_api::proxy_change::infra::ChangePasswordProxyRequestDecoder;

use crate::z_details::_api::message::data::MessageError;

pub struct ChangeProxyRequestDecoder {
    body: String,
}

impl ChangeProxyRequestDecoder {
    pub const fn new(body: String) -> Self {
        Self { body }
    }
}

impl ChangePasswordProxyRequestDecoder for ChangeProxyRequestDecoder {
    fn decode(self) -> Result<ChangePasswordFieldsExtract, MessageError> {
        let message =
            ChangePasswordPb::decode(decode_base64(self.body)?).map_err(invalid_protobuf)?;

        Ok(ChangePasswordFieldsExtract {
            current_password: message.current_password,
            new_password: message.new_password,
        })
    }
}
