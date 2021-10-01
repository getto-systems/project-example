use crate::{
    auth::ticket::remote::encode::data::AuthTicketEncoded,
    z_lib::remote::message::data::MessageError,
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