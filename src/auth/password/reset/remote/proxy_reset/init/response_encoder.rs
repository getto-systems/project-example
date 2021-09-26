use crate::z_details::_api::message::helper::encode_protobuf_base64;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideCookie;

use crate::auth::password::reset::_api::y_protobuf::api::{
    ResetPasswordErrorKindPb, ResetPasswordErrorPb, ResetPasswordResultPb,
};

use crate::auth::auth_ticket::_api::kernel::init::response_builder::CookieAuthTokenResponseBuilder;

use crate::auth::_api::proxy::AuthProxyResponseEncoder;

use crate::auth::{
    auth_ticket::_api::kernel::infra::AuthTokenResponseBuilder,
    password::reset::remote::proxy_reset::infra::ResetPasswordProxyResponse,
};

use crate::{
    auth::{
        auth_ticket::_api::kernel::data::AuthTokenMessage,
        password::reset::remote::proxy_reset::data::{
            ResetPasswordProxyMessage, ResetPasswordProxyMessageEncoded, ResetPasswordProxyResult,
        },
    },
    z_details::_api::message::data::MessageError,
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
                let message = ResetPasswordResultPb {
                    success: false,
                    err: Some(ResetPasswordErrorPb {
                        kind: ResetPasswordErrorKindPb::InvalidReset as i32,
                    }),
                    ..Default::default()
                };
                Ok(ResetPasswordProxyResult::InvalidReset(
                    encode_protobuf_base64(message)?,
                ))
            }
            ResetPasswordProxyResponse::AlreadyReset => {
                let message = ResetPasswordResultPb {
                    success: false,
                    err: Some(ResetPasswordErrorPb {
                        kind: ResetPasswordErrorKindPb::AlreadyReset as i32,
                    }),
                    ..Default::default()
                };
                Ok(ResetPasswordProxyResult::AlreadyReset(
                    encode_protobuf_base64(message)?,
                ))
            }
            ResetPasswordProxyResponse::Success(ticket) => {
                let message = ResetPasswordResultPb {
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
