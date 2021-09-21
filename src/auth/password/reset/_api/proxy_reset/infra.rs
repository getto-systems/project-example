use crate::auth::password::reset::_common::reset::infra::ResetPasswordFieldsExtract;

use crate::{
    auth::auth_ticket::_common::encode::data::AuthTicketEncoded,
    z_details::_api::message::data::MessageError,
};

pub enum ResetPasswordProxyResponse {
    Success(AuthTicketEncoded),
    InvalidReset,
    AlreadyReset,
}

pub trait ResetPasswordProxyRequestDecoder {
    fn decode(self) -> Result<ResetPasswordFieldsExtract, MessageError>;
}
