use crate::auth::auth_ticket::_api::kernel::data::{AuthTokenMessage, AuthTokenMessageEncoded};

pub type AuthenticatePasswordMessage = AuthenticatePasswordResult<AuthTokenMessage>;
pub type AuthenticatePasswordMessageEncoded = AuthenticatePasswordResult<AuthTokenMessageEncoded>;

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
