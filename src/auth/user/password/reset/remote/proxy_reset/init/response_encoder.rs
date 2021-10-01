use crate::z_lib::remote::message::helper::encode_protobuf_base64;

use crate::auth::user::password::reset::remote::y_protobuf::api::{
    ResetPasswordApiErrorKindPb, ResetPasswordApiErrorPb, ResetPasswordApiResponsePb,
};

use crate::auth::remote::x_outside_feature::api::feature::AuthOutsideCookie;

use crate::auth::ticket::remote::kernel::init::response_builder::CookieAuthTokenResponseBuilder;

use crate::auth::remote::service::proxy::AuthProxyResponseEncoder;

use crate::auth::{
    ticket::remote::kernel::infra::AuthTokenResponseBuilder,
    user::password::reset::remote::proxy_reset::infra::ResetPasswordProxyResponse,
};

use crate::{
    auth::{
        ticket::remote::kernel::data::AuthTokenMessage,
        user::password::reset::remote::proxy_reset::data::{
            ResetPasswordProxyMessage, ResetPasswordProxyMessageEncoded, ResetPasswordProxyResult,
        },
    },
    z_lib::remote::message::data::MessageError,
};

pub struct ResponseEncoder<'a> {
    response_builder: CookieAuthTokenResponseBuilder<'a>,
}

impl<'a> ResponseEncoder<'a> {
    pub const fn new(feature: &'a AuthOutsideCookie) -> Self {
        Self {
            response_builder: CookieAuthTokenResponseBuilder::new(feature),
        }
    }

    fn encode_message(
        &self,
        response: ResetPasswordProxyResponse,
    ) -> Result<ResetPasswordProxyMessageEncoded, MessageError> {
        match response {
            ResetPasswordProxyResponse::InvalidReset => {
                let message = ResetPasswordApiResponsePb {
                    success: false,
                    err: Some(ResetPasswordApiErrorPb {
                        kind: ResetPasswordApiErrorKindPb::InvalidReset as i32,
                    }),
                    ..Default::default()
                };
                Ok(ResetPasswordProxyResult::InvalidReset(
                    encode_protobuf_base64(message)?,
                ))
            }
            ResetPasswordProxyResponse::AlreadyReset => {
                let message = ResetPasswordApiResponsePb {
                    success: false,
                    err: Some(ResetPasswordApiErrorPb {
                        kind: ResetPasswordApiErrorKindPb::AlreadyReset as i32,
                    }),
                    ..Default::default()
                };
                Ok(ResetPasswordProxyResult::AlreadyReset(
                    encode_protobuf_base64(message)?,
                ))
            }
            ResetPasswordProxyResponse::Success(ticket) => {
                let message = ResetPasswordApiResponsePb {
                    success: true,
                    value: Some(ticket.user.into()),
                    ..Default::default()
                };
                Ok(ResetPasswordProxyResult::Success(AuthTokenMessage {
                    body: encode_protobuf_base64(message)?,
                    token: ticket.token,
                }))
            }
        }
    }
}

impl<'a> AuthProxyResponseEncoder<ResetPasswordProxyResponse, ResetPasswordProxyMessage>
    for ResponseEncoder<'a>
{
    fn encode(
        &self,
        response: ResetPasswordProxyResponse,
    ) -> Result<ResetPasswordProxyMessage, MessageError> {
        let message = self.encode_message(response)?;
        Ok(message.map(|message| self.response_builder.build(message)))
    }
}
