use crate::auth::password::_api::y_protobuf::api::{
    AuthenticatePasswordErrorKindPb, AuthenticatePasswordErrorPb, AuthenticatePasswordResultPb,
};

use crate::auth::_api::x_outside_feature::feature::AuthOutsideCookie;

use crate::z_details::_api::message::helper::encode_protobuf_base64;

use crate::auth::auth_ticket::_api::kernel::init::response_builder::CookieAuthTokenResponseBuilder;

use crate::auth::_api::proxy::AuthProxyResponseEncoder;

use crate::auth::{
    auth_ticket::_api::kernel::infra::AuthTokenResponseBuilder,
    password::remote::proxy_authenticate::infra::AuthenticatePasswordProxyResponse,
};

use crate::{
    auth::{
        auth_ticket::_api::kernel::data::AuthTokenMessage,
        password::remote::proxy_authenticate::data::{
            AuthenticatePasswordProxyMessage, AuthenticatePasswordProxyMessageEncoded,
            AuthenticatePasswordProxyResult,
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
        response: AuthenticatePasswordProxyResponse,
    ) -> Result<AuthenticatePasswordProxyMessageEncoded, MessageError> {
        match response {
            AuthenticatePasswordProxyResponse::InvalidPassword => {
                let message = AuthenticatePasswordResultPb {
                    success: false,
                    err: Some(AuthenticatePasswordErrorPb {
                        kind: AuthenticatePasswordErrorKindPb::InvalidPassword as i32,
                    }),
                    ..Default::default()
                };
                Ok(AuthenticatePasswordProxyResult::InvalidPassword(
                    encode_protobuf_base64(message)?,
                ))
            }
            AuthenticatePasswordProxyResponse::Success(ticket) => {
                let message = AuthenticatePasswordResultPb {
                    success: true,
                    value: Some(ticket.user.into()),
                    ..Default::default()
                };
                Ok(AuthenticatePasswordProxyResult::Success(AuthTokenMessage {
                    body: encode_protobuf_base64(message)?,
                    token: ticket.token,
                }))
            }
        }
    }
}

impl<'a> AuthProxyResponseEncoder<AuthenticatePasswordProxyResponse, AuthenticatePasswordProxyMessage>
    for ResponseEncoder<'a>
{
    fn encode(
        &self,
        response: AuthenticatePasswordProxyResponse,
    ) -> Result<AuthenticatePasswordProxyMessage, MessageError> {
        let message = self.encode_message(response)?;
        Ok(message.map(|message| self.response_builder.build(message)))
    }
}
