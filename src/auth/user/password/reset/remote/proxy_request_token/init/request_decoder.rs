use prost::Message;

use crate::z_lib::remote::message::helper::{decode_base64, invalid_protobuf};

use crate::auth::user::password::reset::remote::y_protobuf::api::RequestResetTokenApiRequestPb;

use crate::auth::user::password::reset::remote::proxy_request_token::infra::{
    RequestResetTokenFieldsExtract, RequestResetTokenProxyRequestDecoder,
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

impl RequestResetTokenProxyRequestDecoder for RequestDecoder {
    fn decode(self) -> Result<RequestResetTokenFieldsExtract, MessageError> {
        let message = RequestResetTokenApiRequestPb::decode(decode_base64(self.body)?)
            .map_err(invalid_protobuf)?;

        Ok(RequestResetTokenFieldsExtract {
            login_id: message.login_id,
        })
    }
}
