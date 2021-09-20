use crate::auth::auth_ticket::_api::kernel::data::AuthTokenResponse;
use crate::auth::password::_api::y_protobuf::api::{
    AuthenticatePasswordErrorKindPb, AuthenticatePasswordErrorPb, AuthenticatePasswordResultPb,
};

use crate::auth::_api::x_outside_feature::feature::AuthOutsideCookie;

use crate::z_details::_api::message::helper::encode_protobuf_base64;

use crate::auth::auth_ticket::_api::kernel::init::response_builder::CookieAuthTokenResponseBuilder;

use crate::auth::_api::proxy::AuthProxyResponseEncoder;

use super::{
    AuthenticatePasswordMessageEncoded, AuthenticatePasswordResponse, AuthenticatePasswordResult,
};
use crate::auth::auth_ticket::_api::kernel::infra::AuthTokenResponseBuilder;

use crate::{
    auth::auth_ticket::_api::kernel::data::AuthTokenMessage,
    z_details::_api::message::data::MessageError,
};

pub struct AuthenticateResponseEncoder<'a> {
    response_builder: CookieAuthTokenResponseBuilder<'a>,
}

impl<'a> AuthenticateResponseEncoder<'a> {
    pub const fn new(feature: &'a AuthOutsideCookie) -> Self {
        Self {
            response_builder: CookieAuthTokenResponseBuilder::new(feature),
        }
    }

    fn encode_message(
        &self,
        response: AuthenticatePasswordResponse,
    ) -> Result<AuthenticatePasswordMessageEncoded, MessageError> {
        match response {
            AuthenticatePasswordResponse::InvalidPassword => {
                let message = AuthenticatePasswordResultPb {
                    success: false,
                    err: Some(AuthenticatePasswordErrorPb {
                        kind: AuthenticatePasswordErrorKindPb::InvalidPassword as i32,
                    }),
                    ..Default::default()
                };
                Ok(AuthenticatePasswordResult::InvalidPassword(
                    encode_protobuf_base64(message)?,
                ))
            }
            AuthenticatePasswordResponse::Success(ticket) => {
                let message = AuthenticatePasswordResultPb {
                    success: true,
                    value: Some(ticket.user.into()),
                    ..Default::default()
                };
                Ok(AuthenticatePasswordResult::Success(AuthTokenMessage {
                    body: encode_protobuf_base64(message)?,
                    token: ticket.token,
                }))
            }
        }
    }
}

impl<'a>
    AuthProxyResponseEncoder<
        AuthenticatePasswordResponse,
        AuthenticatePasswordResult<AuthTokenResponse>,
    > for AuthenticateResponseEncoder<'a>
{
    fn encode(
        &self,
        response: AuthenticatePasswordResponse,
    ) -> Result<AuthenticatePasswordResult<AuthTokenResponse>, MessageError> {
        let message = self.encode_message(response)?;
        Ok(message.map(|message| self.response_builder.build(message)))
    }
}
