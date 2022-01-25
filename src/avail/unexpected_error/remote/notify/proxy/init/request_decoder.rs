use prost::Message;

use crate::z_lib::remote::message::helper::{decode_base64, invalid_protobuf};

use crate::avail::unexpected_error::remote::y_protobuf::api::NotifyUnexpectedErrorApiRequestPb;

use crate::avail::unexpected_error::remote::notify::{
    infra::NotifyUnexpectedErrorFieldsExtract,
    proxy::infra::NotifyUnexpectedErrorProxyRequestDecoder,
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

impl NotifyUnexpectedErrorProxyRequestDecoder for RequestDecoder {
    fn decode(self) -> Result<NotifyUnexpectedErrorFieldsExtract, MessageError> {
        let message = NotifyUnexpectedErrorApiRequestPb::decode(decode_base64(self.body)?)
            .map_err(invalid_protobuf)?;

        Ok(NotifyUnexpectedErrorFieldsExtract { err: message.json })
    }
}