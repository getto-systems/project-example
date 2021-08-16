use crate::auth::auth_ticket::_common::kernel::data::AuthTokenEncoded;

pub struct AuthTokenResponse {
    pub domain: String,
    pub message: AuthTokenMessage,
}

pub struct AuthTokenMessage {
    pub body: String,
    pub token: AuthTokenEncoded,
}
