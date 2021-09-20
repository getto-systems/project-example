use crate::auth::auth_ticket::{
    _api::kernel::data::{AuthTokenMessage, AuthTokenResponse},
    _common::encode::data::AuthTicketEncoded,
};

pub type AuthenticatePasswordMessage = AuthenticatePasswordResult<AuthTokenResponse>;
pub type AuthenticatePasswordMessageEncoded = AuthenticatePasswordResult<AuthTokenMessage>;

pub enum AuthenticatePasswordResult<T> {
    Success(T),
    InvalidPassword(String),
}

impl<T> AuthenticatePasswordResult<T> {
    pub fn map<M>(self, mapper: impl Fn(T) -> M) -> AuthenticatePasswordResult<M> {
        match self {
            Self::InvalidPassword(response) => {
                AuthenticatePasswordResult::InvalidPassword(response)
            }
            Self::Success(response) => AuthenticatePasswordResult::Success(mapper(response)),
        }
    }
}

pub enum AuthenticatePasswordResponse {
    Success(AuthTicketEncoded),
    InvalidPassword,
}
