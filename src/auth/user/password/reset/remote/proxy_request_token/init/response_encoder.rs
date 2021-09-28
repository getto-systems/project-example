use crate::z_lib::remote::message::helper::encode_protobuf_base64;

use crate::auth::user::password::reset::_api::y_protobuf::api::{
    RequestResetTokenErrorKindPb, RequestResetTokenErrorPb, RequestResetTokenResultPb,
};

use crate::auth::remote::service::proxy::AuthProxyResponseEncoder;

use crate::auth::user::password::reset::remote::proxy_request_token::infra::RequestResetTokenProxyResponse;

use crate::{
    auth::user::password::reset::remote::proxy_request_token::data::RequestResetTokenProxyMessage,
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
