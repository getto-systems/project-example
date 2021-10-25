use crate::auth::user::password::remote::authenticate::infra::AuthenticatePasswordFieldsExtract;

use crate::{
    auth::ticket::remote::encode::data::AuthTicketEncoded,
    z_lib::remote::message::data::MessageError,
};

pub enum AuthenticatePasswordProxyResponse {
    Success(AuthTicketEncoded),
    InvalidPassword,
}

pub trait AuthenticatePasswordProxyRequestDecoder {
    fn decode(self) -> Result<AuthenticatePasswordFieldsExtract, MessageError>;
}
