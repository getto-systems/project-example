use crate::auth::auth_ticket::_common::kernel::data::AuthTokenEncoded;

pub struct AuthTokenMessage {
    pub domain: String,
    pub message: AuthTokenMessageEncoded,
}

pub struct AuthTokenMessageEncoded {
    pub message: String,
    pub token: AuthTokenEncoded,
}
