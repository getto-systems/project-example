use crate::z_details::_api::message::helper::encode_protobuf_base64;

use crate::auth::user::password::_api::y_protobuf::api::{
    AuthenticatePasswordErrorKindPb, ChangePasswordErrorPb, ChangePasswordResultPb,
};

use crate::auth::remote::service::proxy::AuthProxyResponseEncoder;

use crate::auth::user::password::remote::proxy_change::infra::ChangePasswordProxyResponse;

use crate::{
    auth::user::password::remote::proxy_change::data::ChangePasswordProxyMessage,
    z_details::_api::message::data::MessageError,
};

pub struct ResponseEncoder;

impl<'a> AuthProxyResponseEncoder<ChangePasswordProxyResponse, ChangePasswordProxyMessage>
    for ResponseEncoder
{
    fn encode(
        &self,
        response: ChangePasswordProxyResponse,
    ) -> Result<ChangePasswordProxyMessage, MessageError> {
        match response {
            ChangePasswordProxyResponse::InvalidPassword => {
                let message = ChangePasswordResultPb {
                    success: false,
                    err: Some(ChangePasswordErrorPb {
                        kind: AuthenticatePasswordErrorKindPb::InvalidPassword as i32,
                    }),
                    ..Default::default()
                };
                Ok(ChangePasswordProxyMessage::InvalidPassword(
                    encode_protobuf_base64(message)?,
                ))
            }
            ChangePasswordProxyResponse::Success => {
                let message = ChangePasswordResultPb {
                    success: true,
                    ..Default::default()
                };
                Ok(ChangePasswordProxyMessage::Success(encode_protobuf_base64(
                    message,
                )?))
            }
        }
    }
}
