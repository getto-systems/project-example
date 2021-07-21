use crate::auth::auth_ticket::_api::y_protobuf::api::AuthenticateResponsePb;

use crate::z_details::_api::message::helper::encode_protobuf_base64;

use crate::auth::auth_ticket::_api::renew::infra::RenewAuthTicketResponseEncoder;

use crate::{
    auth::auth_ticket::{
        _api::kernel::data::AuthTokenMessageEncoded,
        _common::encode::data::EncodeAuthTicketResponse,
    },
    z_details::_api::message::data::MessageError,
};

pub struct ProstRenewAuthTicketResponseEncoder;

impl RenewAuthTicketResponseEncoder for ProstRenewAuthTicketResponseEncoder {
    fn encode(
        &self,
        response: EncodeAuthTicketResponse,
    ) -> Result<AuthTokenMessageEncoded, MessageError> {
        let (user, token) = response.extract();
        let message: AuthenticateResponsePb = user.into();
        Ok(AuthTokenMessageEncoded {
            message: encode_protobuf_base64(message)?,
            token,
        })
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::auth_ticket::_api::renew::infra::RenewAuthTicketResponseEncoder;

    use crate::{
        auth::auth_ticket::{
            _api::kernel::data::AuthTokenMessageEncoded,
            _common::encode::data::EncodeAuthTicketResponse,
        },
        z_details::_api::message::data::MessageError,
    };

    pub struct StaticRenewAuthTicketResponseEncoder;

    impl RenewAuthTicketResponseEncoder for StaticRenewAuthTicketResponseEncoder {
        fn encode(
            &self,
            response: EncodeAuthTicketResponse,
        ) -> Result<AuthTokenMessageEncoded, MessageError> {
            let (_user, token) = response.extract();
            Ok(AuthTokenMessageEncoded {
                message: "ENCODED".into(),
                token,
            })
        }
    }
}
