use crate::z_details::_api::message::helper::encode_protobuf_base64;

use crate::auth::password::reset::_api::y_protobuf::api::{
    RequestResetTokenErrorKindPb, RequestResetTokenErrorPb, RequestResetTokenResultPb,
};

use crate::auth::_api::proxy::AuthProxyResponseEncoder;

use crate::auth::password::reset::_api::proxy_request_token::infra::RequestResetTokenProxyResponse;

use crate::{
    auth::password::reset::_api::proxy_request_token::data::RequestResetTokenProxyMessage,
    z_details::_api::message::data::MessageError,
};

pub struct ResponseEncoder;

impl<'a> AuthProxyResponseEncoder<RequestResetTokenProxyResponse, RequestResetTokenProxyMessage>
    for ResponseEncoder
{
    fn encode(
        &self,
        response: RequestResetTokenProxyResponse,
    ) -> Result<RequestResetTokenProxyMessage, MessageError> {
        match response {
            RequestResetTokenProxyResponse::InvalidRequest => {
                let message = RequestResetTokenResultPb {
                    success: false,
                    err: Some(RequestResetTokenErrorPb {
                        kind: RequestResetTokenErrorKindPb::InvalidRequest as i32,
                    }),
                    ..Default::default()
                };
                Ok(RequestResetTokenProxyMessage::InvalidRequest(
                    encode_protobuf_base64(message)?,
                ))
            }
            RequestResetTokenProxyResponse::Success => {
                let message = RequestResetTokenResultPb {
                    success: true,
                    ..Default::default()
                };
                Ok(RequestResetTokenProxyMessage::Success(
                    encode_protobuf_base64(message)?,
                ))
            }
        }
    }
}
