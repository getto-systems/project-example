use crate::auth::auth_ticket::_api::y_protobuf::api::AuthenticateResponsePb;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideCookie;

use crate::z_details::_api::message::helper::encode_protobuf_base64;

use crate::auth::auth_ticket::_api::renew::infra::RenewAuthTicketMessenger;

use crate::{
    auth::auth_ticket::{
        _api::renew::data::AuthTokenMessage, _common::encode::data::EncodeAuthTicketResponse,
    },
    z_details::_api::message::data::MessageError,
};

pub struct ProstRenewAuthTicketMessenger<'a> {
    domain: &'a str,
}

impl<'a> ProstRenewAuthTicketMessenger<'a> {
    pub const fn new(feature: &'a AuthOutsideCookie) -> Self {
        Self {
            domain: feature.domain,
        }
    }
}

impl<'a> RenewAuthTicketMessenger for ProstRenewAuthTicketMessenger<'a> {
    fn encode(&self, response: EncodeAuthTicketResponse) -> Result<AuthTokenMessage, MessageError> {
        let (user, token) = response.extract();
        let message: AuthenticateResponsePb = user.into();
        Ok(AuthTokenMessage {
            domain: self.domain.into(),
            message: encode_protobuf_base64(message)?,
            token,
        })
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::auth_ticket::_api::renew::infra::RenewAuthTicketMessenger;

    use crate::{
        auth::auth_ticket::{
            _api::renew::data::AuthTokenMessage, _common::encode::data::EncodeAuthTicketResponse,
        },
        z_details::_api::message::data::MessageError,
    };

    pub struct StaticRenewAuthTicketMessenger;

    impl RenewAuthTicketMessenger for StaticRenewAuthTicketMessenger {
        fn encode(
            &self,
            response: EncodeAuthTicketResponse,
        ) -> Result<AuthTokenMessage, MessageError> {
            let (_user, token) = response.extract();
            Ok(AuthTokenMessage {
                domain: "DOMAIN".into(),
                message: "MESSAGE".into(),
                token,
            })
        }
    }
}
