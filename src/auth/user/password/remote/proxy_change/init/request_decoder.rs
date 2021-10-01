use prost::Message;

use crate::auth::user::password::remote::y_protobuf::api::ChangePasswordApiRequestPb;

use crate::z_lib::remote::message::helper::{decode_base64, invalid_protobuf};

use crate::auth::user::password::remote::proxy_change::infra::{
    ChangePasswordFieldsExtract, ChangePasswordProxyRequestDecoder,
};

use crate::z_lib::remote::message::data::MessageError;

pub struct RequestDecoder {
    body: String,
}

impl RequestDecoder {
    pub const fn new(body: String) -> Self {
        Self { body }
    }
}

impl ChangePasswordProxyRequestDecoder for RequestDecoder {
    fn decode(self) -> Result<ChangePasswordFieldsExtract, MessageError> {
        let message = ChangePasswordApiRequestPb::decode(decode_base64(self.body)?)
            .map_err(invalid_protobuf)?;

        Ok(ChangePasswordFieldsExtract {
            current_password: message.current_password,
            new_password: message.new_password,
        })
    }
}
