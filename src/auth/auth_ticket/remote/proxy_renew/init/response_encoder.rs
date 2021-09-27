use crate::z_details::_api::message::helper::encode_protobuf_base64;

use crate::auth::auth_ticket::_api::y_protobuf::api::AuthenticateResponsePb;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideCookie;

use crate::auth::auth_ticket::_api::kernel::init::response_builder::CookieAuthTokenResponseBuilder;

use crate::auth::_api::proxy::AuthProxyResponseEncoder;

use crate::auth::auth_ticket::_api::kernel::infra::AuthTokenResponseBuilder;

use crate::{
    auth::auth_ticket::{
        _api::kernel::data::{AuthTokenMessage, AuthTokenResponse},
        remote::encode::data::AuthTicketEncoded,
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
}

impl<'a> AuthProxyResponseEncoder<AuthTicketEncoded, AuthTokenResponse>
    for ResponseEncoder<'a>
{
    fn encode(&self, ticket: AuthTicketEncoded) -> Result<AuthTokenResponse, MessageError> {
        let message: AuthenticateResponsePb = ticket.user.into();
        let message = AuthTokenMessage {
            body: encode_protobuf_base64(message)?,
            token: ticket.token,
        };
        Ok(self.response_builder.build(message))
    }
}
