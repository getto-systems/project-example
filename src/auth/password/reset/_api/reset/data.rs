use crate::auth::auth_ticket::_api::kernel::data::{AuthTokenMessage, AuthTokenMessageEncoded};

pub type ResetPasswordMessage = ResetPasswordResult<AuthTokenMessage>;
pub type ResetPasswordMessageEncoded = ResetPasswordResult<AuthTokenMessageEncoded>;

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
