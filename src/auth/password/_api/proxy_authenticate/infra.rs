use crate::auth::password::_common::authenticate::infra::AuthenticatePasswordFieldsExtract;

use crate::{
    auth::auth_ticket::_common::encode::data::AuthTicketEncoded,
    z_details::_api::message::data::MessageError,
};

pub enum AuthenticatePasswordProxyResponse {
    Success(AuthTicketEncoded),
    InvalidPassword,
}

pub trait AuthenticatePasswordProxyRequestDecoder {
    fn decode(self) -> Result<AuthenticatePasswordFieldsExtract, MessageError>;
}
