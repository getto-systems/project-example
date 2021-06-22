pub mod messenger;

use crate::auth::{
    auth_ticket::_api::kernel::infra::{AuthClock, CheckAuthNonceInfra},
    auth_user::_api::kernel::infra::AuthUserRepository,
    password::_api::kernel::infra::{
        AuthUserPasswordMatcher, AuthUserPasswordRepository, PlainPassword,
    },
};

use crate::auth::password::_api::authenticate::data::AuthenticatePasswordResponse;
use crate::z_details::_api::message::data::MessageError;

pub trait AuthenticatePasswordInfra {
    type CheckNonceInfra: CheckAuthNonceInfra;
    type Clock: AuthClock;
    type PasswordMatcher: AuthUserPasswordMatcher;
    type PasswordRepository: AuthUserPasswordRepository;
    type UserRepository: AuthUserRepository;
    type Messenger: AuthenticatePasswordMessenger;

    fn check_nonce_infra(&self) -> &Self::CheckNonceInfra;
    fn clock(&self) -> &Self::Clock;
    fn password_matcher(&self, plain_password: PlainPassword) -> Self::PasswordMatcher {
        Self::PasswordMatcher::new(plain_password)
    }
    fn password_repository(&self) -> &Self::PasswordRepository;
    fn user_repository(&self) -> &Self::UserRepository;
    fn messenger(&self) -> &Self::Messenger;
}

pub trait AuthenticatePasswordMessenger {
    fn decode(&self) -> Result<AuthenticatePasswordFieldsExtract, MessageError>;
    fn encode_invalid_password(&self) -> Result<AuthenticatePasswordResponse, MessageError>;
}

#[derive(Clone)]
pub struct AuthenticatePasswordFieldsExtract {
    pub login_id: String,
    pub password: String,
}
