use prost::Message;

use crate::auth::password::reset::_api::y_protobuf::api::RequestResetTokenPb;

use crate::z_details::_api::message::helper::{decode_base64, invalid_protobuf};

use crate::auth::password::reset::{
    _api::request_token::infra::RequestResetTokenRequestDecoder,
    _common::request_token::infra::RequestResetTokenFieldsExtract,
};

use crate::z_details::_api::message::data::MessageError;

pub struct ProtobufRequestResetTokenRequestDecoder {
    body: String,
}

impl ProtobufRequestResetTokenRequestDecoder {
    pub const fn new(body: String) -> Self {
        Self { body }
    }
}

impl RequestResetTokenRequestDecoder for ProtobufRequestResetTokenRequestDecoder {
    fn decode(self) -> Result<RequestResetTokenFieldsExtract, MessageError> {
        let message =
            RequestResetTokenPb::decode(decode_base64(self.body)?).map_err(invalid_protobuf)?;

        Ok(RequestResetTokenFieldsExtract {
            login_id: message.login_id,
        })
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::password::reset::{
        _api::request_token::infra::RequestResetTokenRequestDecoder,
        _common::request_token::infra::RequestResetTokenFieldsExtract,
    };

    use crate::z_details::_api::message::data::MessageError;

    pub struct StaticRequestResetTokenRequestDecoder {
        pub fields: RequestResetTokenFieldsExtract,
    }

    impl RequestResetTokenRequestDecoder for StaticRequestResetTokenRequestDecoder {
        fn decode(self) -> Result<RequestResetTokenFieldsExtract, MessageError> {
            Ok(self.fields)
        }
    }
}
