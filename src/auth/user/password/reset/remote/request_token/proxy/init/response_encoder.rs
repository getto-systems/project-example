use crate::z_lib::remote::message::helper::encode_protobuf_base64;

use crate::auth::user::password::reset::remote::y_protobuf::api::{
    RequestResetTokenApiErrorKindPb, RequestResetTokenApiErrorPb, RequestResetTokenApiResponsePb,
};

use crate::auth::remote::service::proxy::AuthProxyResponseEncoder;

use crate::auth::user::password::reset::remote::request_token::proxy::infra::RequestResetTokenProxyResponse;

use crate::{
    auth::user::password::reset::remote::request_token::proxy::data::RequestResetTokenProxyMessage,
    z_lib::remote::message::data::MessageError,
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
                let message = RequestResetTokenApiResponsePb {
                    success: false,
                    err: Some(RequestResetTokenApiErrorPb {
                        kind: RequestResetTokenApiErrorKindPb::InvalidRequest as i32,
                    }),
                    ..Default::default()
                };
                Ok(RequestResetTokenProxyMessage::InvalidRequest(
                    encode_protobuf_base64(message)?,
                ))
            }
            RequestResetTokenProxyResponse::Success => {
                let message = RequestResetTokenApiResponsePb {
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