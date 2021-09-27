use crate::auth::auth_ticket::remote::kernel::data::{AuthTokenMessage, AuthTokenResponse};

pub type AuthenticatePasswordProxyMessage = AuthenticatePasswordProxyResult<AuthTokenResponse>;
pub type AuthenticatePasswordProxyMessageEncoded =
    AuthenticatePasswordProxyResult<AuthTokenMessage>;

pub enum AuthenticatePasswordProxyResult<T> {
    Success(T),
    InvalidPassword(String),
}

impl<T> AuthenticatePasswordProxyResult<T> {
    pub fn map<M>(self, mapper: impl Fn(T) -> M) -> AuthenticatePasswordProxyResult<M> {
        match self {
            Self::InvalidPassword(response) => {
                AuthenticatePasswordProxyResult::InvalidPassword(response)
            }
            Self::Success(response) => AuthenticatePasswordProxyResult::Success(mapper(response)),
        }
    }
}
