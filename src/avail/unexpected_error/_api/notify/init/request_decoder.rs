use prost::Message;

use crate::avail::unexpected_error::_api::y_protobuf::api::NotifyUnexpectedErrorPb;

use crate::z_details::_api::message::helper::{decode_base64, invalid_protobuf};

use crate::avail::unexpected_error::_api::notify::infra::NotifyUnexpectedErrorRequestDecoder;

use crate::z_details::_api::message::data::MessageError;

pub struct ProstNotifyUnexpectedErrorRequestDecoder {
    body: String,
}

impl ProstNotifyUnexpectedErrorRequestDecoder {
    pub const fn new(body: String) -> Self {
        Self { body }
    }
}

impl NotifyUnexpectedErrorRequestDecoder for ProstNotifyUnexpectedErrorRequestDecoder {
    fn decode(self) -> Result<String, MessageError> {
        let message =
            NotifyUnexpectedErrorPb::decode(decode_base64(self.body)?).map_err(invalid_protobuf)?;

        Ok(message.json)
    }
}

#[cfg(test)]
pub mod test {
    use crate::avail::unexpected_error::_api::notify::infra::NotifyUnexpectedErrorRequestDecoder;

    use crate::z_details::_api::message::data::MessageError;

    pub struct StaticNotifyUnexpectedErrorRequestDecoder {
        pub err: String,
    }

    impl NotifyUnexpectedErrorRequestDecoder for StaticNotifyUnexpectedErrorRequestDecoder {
        fn decode(self) -> Result<String, MessageError> {
            Ok(self.err)
        }
    }
}
