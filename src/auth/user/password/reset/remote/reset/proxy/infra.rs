use crate::auth::user::password::reset::remote::reset::infra::ResetPasswordFieldsExtract;

use crate::{
    auth::ticket::remote::encode::data::AuthTicketEncoded,
    z_lib::remote::message::data::MessageError,
};

pub enum ResetPasswordProxyResponse {
    Success(AuthTicketEncoded),
    InvalidReset,
    AlreadyReset,
}

pub trait ResetPasswordProxyRequestDecoder {
    fn decode(self) -> Result<ResetPasswordFieldsExtract, MessageError>;
}
