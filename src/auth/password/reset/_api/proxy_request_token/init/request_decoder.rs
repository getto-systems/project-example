use prost::Message;

use crate::z_details::_api::message::helper::{decode_base64, invalid_protobuf};

use crate::auth::password::reset::_api::y_protobuf::api::RequestResetTokenPb;

use crate::auth::password::reset::{
    _api::proxy_request_token::infra::RequestResetTokenProxyRequestDecoder,
    _common::request_token::infra::RequestResetTokenFieldsExtract,
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

impl RequestResetTokenProxyRequestDecoder for RequestDecoder {
    fn decode(self) -> Result<RequestResetTokenFieldsExtract, MessageError> {
        let message =
            RequestResetTokenPb::decode(decode_base64(self.body)?).map_err(invalid_protobuf)?;

        Ok(RequestResetTokenFieldsExtract {
            login_id: message.login_id,
        })
    }
}
