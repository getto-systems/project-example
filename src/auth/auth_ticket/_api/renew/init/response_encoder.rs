use crate::auth::auth_ticket::_api::y_protobuf::api::AuthenticateResponsePb;

use crate::z_details::_api::message::helper::encode_protobuf_base64;

use crate::auth::auth_ticket::_api::renew::infra::RenewAuthTicketResponseEncoder;

use crate::{
    auth::auth_ticket::{
        _api::kernel::data::AuthTokenMessage,
        _common::encode::data::AuthTicketEncoded,
    },
    z_details::_api::message::data::MessageError,
};

pub struct ProstRenewAuthTicketResponseEncoder;

impl RenewAuthTicketResponseEncoder for ProstRenewAuthTicketResponseEncoder {
    fn encode(
        &self,
        ticket: AuthTicketEncoded,
    ) -> Result<AuthTokenMessage, MessageError> {
        let message: AuthenticateResponsePb = ticket.user.into();
        Ok(AuthTokenMessage {
            body: encode_protobuf_base64(message)?,
            token: ticket.token,
        })
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::auth_ticket::_api::renew::infra::RenewAuthTicketResponseEncoder;

    use crate::{
        auth::auth_ticket::{
            _api::kernel::data::AuthTokenMessage,
            _common::encode::data::AuthTicketEncoded,
        },
        z_details::_api::message::data::MessageError,
    };

    pub struct StaticRenewAuthTicketResponseEncoder;

    impl RenewAuthTicketResponseEncoder for StaticRenewAuthTicketResponseEncoder {
        fn encode(
            &self,
            ticket: AuthTicketEncoded,
        ) -> Result<AuthTokenMessage, MessageError> {
            Ok(AuthTokenMessage {
                body: "ENCODED".into(),
                token: ticket.token,
            })
        }
    }
}
