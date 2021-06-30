use crate::auth::{
    auth_ticket::_api::kernel::infra::CheckAuthNonceInfra,
    auth_user::_api::kernel::infra::AuthUserRepository,
    password::_api::kernel::infra::{
        AuthUserPasswordMatcher, AuthUserPasswordRepository, PlainPassword, VerifyPasswordError,
    },
};

use crate::auth::password::_api::authenticate::event::AuthenticatePasswordEvent;

use crate::auth::password::_api::authenticate::data::AuthenticatePasswordResponse;
use crate::z_details::_api::message::data::MessageError;

pub trait AuthenticatePasswordInfra {
    type CheckNonceInfra: CheckAuthNonceInfra;
    type PasswordMatcher: AuthUserPasswordMatcher;
    type PasswordRepository: AuthUserPasswordRepository;
    type UserRepository: AuthUserRepository;
    type Messenger: AuthenticatePasswordMessenger;

    fn check_nonce_infra(&self) -> &Self::CheckNonceInfra;
    fn password_matcher(&self, plain_password: PlainPassword) -> Self::PasswordMatcher {
        Self::PasswordMatcher::new(plain_password)
    }
    fn password_repository(&self) -> &Self::PasswordRepository;
    fn user_repository(&self) -> &Self::UserRepository;
    fn messenger(&self) -> &Self::Messenger;
}

pub trait AuthenticatePasswordMessenger {
    fn decode(&self) -> Result<AuthenticatePasswordFieldsExtract, MessageError>;
    fn encode_user_not_found(&self) -> Result<AuthenticatePasswordResponse, MessageError>;
    fn encode_password_not_found(&self) -> Result<AuthenticatePasswordResponse, MessageError>;
    fn encode_password_not_matched(&self) -> Result<AuthenticatePasswordResponse, MessageError>;
}

#[derive(Clone)]
pub struct AuthenticatePasswordFieldsExtract {
    pub login_id: String,
    pub password: String,
}

impl VerifyPasswordError {
    pub fn into_authenticate_password_event(
        self,
        messenger: &impl AuthenticatePasswordMessenger,
    ) -> AuthenticatePasswordEvent {
        match self {
            Self::PasswordHashError(err) => AuthenticatePasswordEvent::PasswordHashError(err),
            Self::RepositoryError(err) => AuthenticatePasswordEvent::RepositoryError(err),
            Self::UserNotFound => messenger.encode_user_not_found().into(),
            Self::PasswordNotFound => messenger.encode_password_not_found().into(),
            Self::PasswordNotMatched => messenger.encode_password_not_matched().into(),
        }
    }
}

impl Into<AuthenticatePasswordEvent> for Result<AuthenticatePasswordResponse, MessageError> {
    fn into(self) -> AuthenticatePasswordEvent {
        match self {
            Ok(response) => AuthenticatePasswordEvent::InvalidPassword(response),
            Err(err) => AuthenticatePasswordEvent::MessageError(err),
        }
    }
}
