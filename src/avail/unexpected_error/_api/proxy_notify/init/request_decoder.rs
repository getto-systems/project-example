use prost::Message;

use crate::z_details::_api::message::helper::{decode_base64, invalid_protobuf};

use crate::avail::unexpected_error::_api::y_protobuf::api::NotifyUnexpectedErrorPb;

use crate::avail::unexpected_error::{
    _api::proxy_notify::infra::NotifyUnexpectedErrorProxyRequestDecoder,
    _common::notify::infra::NotifyUnexpectedErrorFieldsExtract,
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

impl NotifyUnexpectedErrorProxyRequestDecoder for RequestDecoder {
    fn decode(self) -> Result<NotifyUnexpectedErrorFieldsExtract, MessageError> {
        let message =
            NotifyUnexpectedErrorPb::decode(decode_base64(self.body)?).map_err(invalid_protobuf)?;

        Ok(NotifyUnexpectedErrorFieldsExtract { err: message.json })
    }
}
