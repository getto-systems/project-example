use prost::Message;

use crate::auth::password::reset::_api::y_protobuf::api::{
    RequestResetTokenErrorKindPb, RequestResetTokenErrorPb, RequestResetTokenPb,
    RequestResetTokenResultPb,
};
use crate::z_details::_api::message::helper::{
    decode_base64, encode_protobuf_base64, invalid_protobuf,
};

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
        let message = RequestResetTokenPb::decode(decode_base64(self.body.clone())?)
            .map_err(invalid_protobuf)?;

        Ok(RequestResetTokenFieldsExtract {
            login_id: message.login_id,
        })
    }
    fn encode_success(&self) -> Result<RequestResetTokenResponse, MessageError> {
        let message = encode_protobuf_base64(RequestResetTokenResultPb {
            success: true,
            ..Default::default()
        })?;
        Ok(RequestResetTokenResponse::Success(message))
    }
    fn encode_destination_not_found(&self) -> Result<RequestResetTokenResponse, MessageError> {
        encode_failed(
            RequestResetTokenErrorKindPb::InvalidRequest,
            RequestResetTokenResponse::DestinationNotFound,
        )
    }
    fn encode_user_not_found(&self) -> Result<RequestResetTokenResponse, MessageError> {
        encode_failed(
            RequestResetTokenErrorKindPb::InvalidRequest,
            RequestResetTokenResponse::UserNotFound,
        )
    }
}

fn encode_failed(
    error_kind: RequestResetTokenErrorKindPb,
    response: impl Fn(String) -> RequestResetTokenResponse,
) -> Result<RequestResetTokenResponse, MessageError> {
    let message = encode_protobuf_base64(RequestResetTokenResultPb {
        success: false,
        err: Some(RequestResetTokenErrorPb {
            kind: error_kind as i32,
        }),
        ..Default::default()
    })?;
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
