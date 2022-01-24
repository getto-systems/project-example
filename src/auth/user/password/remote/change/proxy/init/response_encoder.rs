use crate::z_lib::remote::message::helper::encode_protobuf_base64;

use crate::auth::user::password::remote::y_protobuf::api::{
    AuthenticatePasswordApiErrorKindPb, ChangePasswordApiErrorPb, ChangePasswordApiResponsePb,
};

use crate::auth::remote::service::proxy::AuthProxyResponseEncoder;

use crate::auth::user::password::remote::change::proxy::infra::ChangePasswordProxyResponse;

use crate::{
    auth::user::password::remote::change::proxy::data::ChangePasswordProxyMessage,
    z_lib::remote::message::data::MessageError,
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
                let message = ChangePasswordApiResponsePb {
                    success: false,
                    err: Some(ChangePasswordApiErrorPb {
                        kind: AuthenticatePasswordApiErrorKindPb::InvalidPassword as i32,
                    }),
                    ..Default::default()
                };
                Ok(ChangePasswordProxyMessage::InvalidPassword(
                    encode_protobuf_base64(message)?,
                ))
            }
            ChangePasswordProxyResponse::Success => {
                let message = ChangePasswordApiResponsePb {
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
