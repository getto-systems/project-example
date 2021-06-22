pub mod messenger;
pub mod token_decoder;

use crate::auth::{
    auth_ticket::_api::kernel::infra::{AuthClock, CheckAuthNonceInfra},
    auth_user::_api::kernel::infra::AuthUserRepository,
    password::_api::kernel::infra::{
        AuthUserPasswordHasher, AuthUserPasswordRepository, PlainPassword,
    },
};

use super::data::DecodeResetTokenError;
use crate::auth::password::{
    _api::kernel::data::ResetToken,
    reset::_api::{kernel::data::ResetTokenEncoded, reset::data::ResetPasswordResponse},
};
use crate::z_details::_api::message::data::MessageError;

pub trait ResetPasswordInfra {
    type CheckNonceInfra: CheckAuthNonceInfra;
    type Clock: AuthClock;
    type PasswordRepository: AuthUserPasswordRepository;
    type UserRepository: AuthUserRepository;
    type PasswordHasher: AuthUserPasswordHasher;
    type TokenDecoder: ResetTokenDecoder;
    type Messenger: ResetPasswordMessenger;

    fn check_nonce_infra(&self) -> &Self::CheckNonceInfra;
    fn clock(&self) -> &Self::Clock;
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
