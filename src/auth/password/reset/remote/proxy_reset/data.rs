use crate::auth::auth_ticket::remote::kernel::data::{AuthTokenMessage, AuthTokenResponse};

pub type ResetPasswordProxyMessage = ResetPasswordProxyResult<AuthTokenResponse>;
pub type ResetPasswordProxyMessageEncoded = ResetPasswordProxyResult<AuthTokenMessage>;

pub enum ResetPasswordProxyResult<T> {
    Success(T),
    InvalidReset(String),
    AlreadyReset(String),
}

impl<T> ResetPasswordProxyResult<T> {
    pub fn map<M>(self, mapper: impl Fn(T) -> M) -> ResetPasswordProxyResult<M> {
        match self {
            Self::InvalidReset(response) => ResetPasswordProxyResult::InvalidReset(response),
            Self::AlreadyReset(response) => ResetPasswordProxyResult::AlreadyReset(response),
            Self::Success(response) => ResetPasswordProxyResult::Success(mapper(response)),
        }
    }
}
