use crate::auth::_api::y_protobuf::api::{
    RequestResetTokenResult_pb, RequestResetTokenResult_pb_Error,
    RequestResetTokenResult_pb_ErrorType, RequestResetToken_pb,
};

use crate::z_details::_api::message::helper::{decode_protobuf_base64, encode_protobuf_base64};

use crate::auth::password::reset::_api::request_token::infra::{
    RequestResetTokenFieldsExtract, RequestResetTokenMessenger,
};

use crate::auth::password::reset::_api::request_token::data::RequestResetTokenResponse;
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
    fn encode_success(&self) -> Result<RequestResetTokenResponse, MessageError> {
        let mut message = RequestResetTokenResult_pb::new();
        message.set_success(true);

        let message = encode_protobuf_base64(message)?;
        Ok(RequestResetTokenResponse::Success(message))
    }
    fn encode_destination_not_found(&self) -> Result<RequestResetTokenResponse, MessageError> {
        encode_failed(
            RequestResetTokenResult_pb_ErrorType::INVALID_RESET,
            RequestResetTokenResponse::DestinationNotFound,
        )
    }
    fn encode_user_not_found(&self) -> Result<RequestResetTokenResponse, MessageError> {
        encode_failed(
            RequestResetTokenResult_pb_ErrorType::INVALID_RESET,
            RequestResetTokenResponse::UserNotFound,
        )
    }
}

fn encode_failed(
    field_type: RequestResetTokenResult_pb_ErrorType,
    response: impl Fn(String) -> RequestResetTokenResponse,
) -> Result<RequestResetTokenResponse, MessageError> {
    let mut message = RequestResetTokenResult_pb::new();
    message.set_success(false);

    let mut err = RequestResetTokenResult_pb_Error::new();
    err.set_field_type(field_type);
    message.set_err(err);

    let message = encode_protobuf_base64(message)?;
    Ok(response(message))
}

#[cfg(test)]
pub mod test {
    use crate::auth::password::reset::_api::request_token::infra::{
        RequestResetTokenFieldsExtract, RequestResetTokenMessenger,
    };

    use crate::auth::password::reset::_api::request_token::data::RequestResetTokenResponse;
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
        fn encode_success(&self) -> Result<RequestResetTokenResponse, MessageError> {
            Ok(RequestResetTokenResponse::Success("encoded".into()))
        }
        fn encode_destination_not_found(&self) -> Result<RequestResetTokenResponse, MessageError> {
            Ok(RequestResetTokenResponse::DestinationNotFound(
                "encoded".into(),
            ))
        }
        fn encode_user_not_found(&self) -> Result<RequestResetTokenResponse, MessageError> {
            Ok(RequestResetTokenResponse::UserNotFound("encoded".into()))
        }
    }
}