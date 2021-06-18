use crate::auth::_api::y_protobuf::api::{
    RequestResetTokenResult_pb, RequestResetTokenResult_pb_Error,
    RequestResetTokenResult_pb_ErrorType, RequestResetToken_pb,
};

use crate::z_details::_api::message::helper::{decode_protobuf_base64, encode_protobuf_base64};

use super::{RequestResetTokenFieldsExtract, RequestResetTokenMessenger};

use crate::z_details::_api::message::data::MessageError;

pub struct ProtobufRequestResetTokenMessenger {
    body: String,
}

impl ProtobufRequestResetTokenMessenger {
    pub const fn new(body: String) -> Self {
        Self { body }
    }
}

impl RequestResetTokenMessenger for ProtobufRequestResetTokenMessenger {
    fn decode(&self) -> Result<RequestResetTokenFieldsExtract, MessageError> {
        let message: RequestResetToken_pb = decode_protobuf_base64(self.body.clone())?;

        Ok(RequestResetTokenFieldsExtract {
            login_id: message.login_id,
        })
    }
    fn encode_success(&self) -> Result<String, MessageError> {
        let mut message = RequestResetTokenResult_pb::new();
        message.set_success(true);

        encode_protobuf_base64(message)
    }
    fn encode_invalid_reset(&self) -> Result<String, MessageError> {
        let mut message = RequestResetTokenResult_pb::new();
        message.set_success(false);

        let mut err = RequestResetTokenResult_pb_Error::new();
        err.set_field_type(RequestResetTokenResult_pb_ErrorType::INVALID_RESET);
        message.set_err(err);

        encode_protobuf_base64(message)
    }
}

#[cfg(test)]
pub mod test {
    use super::super::{RequestResetTokenFieldsExtract, RequestResetTokenMessenger};

    use crate::z_details::_api::message::data::MessageError;

    pub struct StaticRequestResetTokenMessenger {
        fields: RequestResetTokenFieldsExtract,
    }

    impl StaticRequestResetTokenMessenger {
        pub const fn new(fields: RequestResetTokenFieldsExtract) -> Self {
            Self { fields }
        }
    }

    impl RequestResetTokenMessenger for StaticRequestResetTokenMessenger {
        fn decode(&self) -> Result<RequestResetTokenFieldsExtract, MessageError> {
            Ok(self.fields.clone())
        }
        fn encode_success(&self) -> Result<String, MessageError> {
            Ok("encoded".into())
        }
        fn encode_invalid_reset(&self) -> Result<String, MessageError> {
            Ok("encoded".into())
        }
    }
}
