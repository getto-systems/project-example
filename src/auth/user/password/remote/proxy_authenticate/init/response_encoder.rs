use crate::auth::user::password::remote::y_protobuf::api::{
    AuthenticatePasswordApiErrorKindPb, AuthenticatePasswordApiErrorPb,
    AuthenticatePasswordApiResponsePb,
};

use crate::auth::remote::x_outside_feature::api::feature::AuthOutsideCookie;

use crate::z_lib::remote::message::helper::encode_protobuf_base64;

use crate::auth::ticket::remote::kernel::init::response_builder::CookieAuthTokenResponseBuilder;

use crate::auth::remote::service::proxy::AuthProxyResponseEncoder;

use crate::auth::{
    ticket::remote::kernel::infra::AuthTokenResponseBuilder,
    user::password::remote::proxy_authenticate::infra::AuthenticatePasswordProxyResponse,
};

use crate::{
    auth::{
        ticket::remote::kernel::data::AuthTokenMessage,
        user::password::remote::proxy_authenticate::data::{
            AuthenticatePasswordProxyMessage, AuthenticatePasswordProxyMessageEncoded,
            AuthenticatePasswordProxyResult,
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
        response: AuthenticatePasswordProxyResponse,
    ) -> Result<AuthenticatePasswordProxyMessageEncoded, MessageError> {
        match response {
            AuthenticatePasswordProxyResponse::InvalidPassword => {
                let message = AuthenticatePasswordApiResponsePb {
                    success: false,
                    err: Some(AuthenticatePasswordApiErrorPb {
                        kind: AuthenticatePasswordApiErrorKindPb::InvalidPassword as i32,
                    }),
                    ..Default::default()
                };
                Ok(AuthenticatePasswordProxyResult::InvalidPassword(
                    encode_protobuf_base64(message)?,
                ))
            }
            AuthenticatePasswordProxyResponse::Success(ticket) => {
                let message = AuthenticatePasswordApiResponsePb {
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

impl<'a>
    AuthProxyResponseEncoder<AuthenticatePasswordProxyResponse, AuthenticatePasswordProxyMessage>
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
