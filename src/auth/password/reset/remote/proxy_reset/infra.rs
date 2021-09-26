use crate::{
    auth::auth_ticket::_common::encode::data::AuthTicketEncoded,
    z_details::_api::message::data::MessageError,
};

pub struct ResetPasswordFieldsExtract {
    pub reset_token: String,
    pub login_id: String,
    pub password: String,
}

pub enum ResetPasswordProxyResponse {
    Success(AuthTicketEncoded),
    InvalidReset,
    AlreadyReset,
}

pub trait ResetPasswordProxyRequestDecoder {
    fn decode(self) -> Result<ResetPasswordFieldsExtract, MessageError>;
}
