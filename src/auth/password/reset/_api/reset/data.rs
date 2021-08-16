use crate::auth::auth_ticket::_api::kernel::data::{AuthTokenResponse, AuthTokenMessage};

pub type ResetPasswordMessage = ResetPasswordResult<AuthTokenResponse>;
pub type ResetPasswordMessageEncoded = ResetPasswordResult<AuthTokenMessage>;

pub enum ResetPasswordResult<T> {
    Success(T),
    InvalidReset(String),
    AlreadyReset(String),
}

impl<T> ResetPasswordResult<T> {
    pub fn map<M>(self, mapper: impl Fn(T) -> M) -> ResetPasswordResult<M> {
        match self {
            Self::InvalidReset(response) => ResetPasswordResult::InvalidReset(response),
            Self::AlreadyReset(response) => ResetPasswordResult::AlreadyReset(response),
            Self::Success(response) => ResetPasswordResult::Success(mapper(response)),
        }
    }
}
