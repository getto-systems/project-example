use crate::auth::{
    auth_ticket::_api::kernel::infra::{AuthClockInfra, CheckAuthNonceInfra},
    auth_user::_api::kernel::infra::AuthUserRepository,
    password::_api::kernel::infra::{
        AuthUserPasswordHasher, AuthUserPasswordRepository, PlainPassword, ResetPasswordError,
    },
};

use crate::auth::password::reset::_api::reset::event::ResetPasswordEvent;

use crate::auth::password::{
    _api::kernel::data::ResetToken,
    reset::_api::{
        kernel::data::ResetTokenEncoded,
        reset::data::{DecodeResetTokenError, ResetPasswordResponse},
    },
};
use crate::z_details::_api::message::data::MessageError;

pub trait ResetPasswordInfra {
    type CheckNonceInfra: CheckAuthNonceInfra;
    type ClockInfra: AuthClockInfra;
    type PasswordRepository: AuthUserPasswordRepository;
    type UserRepository: AuthUserRepository;
    type PasswordHasher: AuthUserPasswordHasher;
    type TokenDecoder: ResetTokenDecoder;
    type Messenger: ResetPasswordMessenger;

    fn check_nonce_infra(&self) -> &Self::CheckNonceInfra;
    fn clock_infra(&self) -> &Self::ClockInfra;
    fn password_repository(&self) -> &Self::PasswordRepository;
    fn user_repository(&self) -> &Self::UserRepository;
    fn password_hasher(&self, plain_password: PlainPassword) -> Self::PasswordHasher {
        Self::PasswordHasher::new(plain_password)
    }
    fn token_decoder(&self) -> &Self::TokenDecoder;
    fn messenger(&self) -> &Self::Messenger;
}

pub trait ResetTokenDecoder {
    fn decode(&self, token: &ResetTokenEncoded) -> Result<ResetToken, DecodeResetTokenError>;
}

pub trait ResetPasswordMessenger {
    fn decode(&self) -> Result<ResetPasswordFieldsExtract, MessageError>;
    fn encode_not_found(&self) -> Result<ResetPasswordResponse, MessageError>;
    fn encode_already_reset(&self) -> Result<ResetPasswordResponse, MessageError>;
    fn encode_expired(&self) -> Result<ResetPasswordResponse, MessageError>;
    fn encode_invalid_login_id(&self) -> Result<ResetPasswordResponse, MessageError>;
}

#[derive(Clone)]
pub struct ResetPasswordFieldsExtract {
    pub login_id: String,
    pub password: String,
    pub reset_token: String,
}

impl ResetPasswordError {
    pub fn into_reset_password_event(
        self,
        messenger: &impl ResetPasswordMessenger,
    ) -> ResetPasswordEvent {
        match self {
            Self::RepositoryError(err) => ResetPasswordEvent::RepositoryError(err),
            Self::PasswordHashError(err) => ResetPasswordEvent::PasswordHashError(err),
            Self::NotFound => messenger.encode_not_found().into(),
            Self::AlreadyReset => messenger.encode_already_reset().into(),
            Self::Expired => messenger.encode_expired().into(),
            Self::InvalidLoginId => messenger.encode_invalid_login_id().into(),
        }
    }
}

impl Into<ResetPasswordEvent> for Result<ResetPasswordResponse, MessageError> {
    fn into(self) -> ResetPasswordEvent {
        match self {
            Ok(response) => ResetPasswordEvent::InvalidReset(response),
            Err(err) => ResetPasswordEvent::MessageError(err),
        }
    }
}
