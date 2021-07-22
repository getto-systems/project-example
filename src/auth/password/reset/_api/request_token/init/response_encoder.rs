use crate::auth::password::reset::_api::y_protobuf::api::{
    RequestResetTokenErrorKindPb, RequestResetTokenErrorPb, RequestResetTokenResultPb,
};

use crate::z_details::_api::message::helper::encode_protobuf_base64;

use crate::auth::password::reset::_api::request_token::infra::{
    RequestResetTokenResponse, RequestResetTokenResponseEncoder,
};

use crate::{
    auth::password::reset::_api::request_token::data::RequestResetTokenResult,
    z_details::_api::message::data::MessageError,
};

pub struct ProstRequestResetTokenResponseEncoder;

impl RequestResetTokenResponseEncoder for ProstRequestResetTokenResponseEncoder {
    fn encode(
        &self,
        response: RequestResetTokenResponse,
    ) -> Result<RequestResetTokenResult, MessageError> {
        match response {
            RequestResetTokenResponse::InvalidRequest => {
                let message = RequestResetTokenResultPb {
                    success: false,
                    err: Some(RequestResetTokenErrorPb {
                        kind: RequestResetTokenErrorKindPb::InvalidRequest as i32,
                    }),
                    ..Default::default()
                };
                Ok(RequestResetTokenResult::InvalidRequest(
                    encode_protobuf_base64(message)?,
                ))
            }
            RequestResetTokenResponse::Success => {
                let message = RequestResetTokenResultPb {
                    success: true,
                    ..Default::default()
                };
                Ok(RequestResetTokenResult::Success(encode_protobuf_base64(
                    message,
                )?))
            }
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::password::reset::_api::request_token::infra::{
        RequestResetTokenResponse, RequestResetTokenResponseEncoder,
    };

    use crate::{
        auth::password::reset::_api::request_token::data::RequestResetTokenResult,
        z_details::_api::message::data::MessageError,
    };

    pub struct StaticRequestResetTokenResponseEncoder;

    impl RequestResetTokenResponseEncoder for StaticRequestResetTokenResponseEncoder {
        fn encode(
            &self,
            response: RequestResetTokenResponse,
        ) -> Result<RequestResetTokenResult, MessageError> {
            match response {
                RequestResetTokenResponse::InvalidRequest => Ok(
                    RequestResetTokenResult::InvalidRequest("INVALID-PASSWORD".into()),
                ),
                RequestResetTokenResponse::Success => {
                    Ok(RequestResetTokenResult::Success("ENCODED".into()))
                }
            }
        }
    }
}
