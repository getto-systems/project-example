pub mod messenger;
pub mod password_hash;
pub mod password_repository;

use std::convert::TryInto;

use crate::auth::{
    auth_ticket::_api::kernel::infra::{
        AuthClock, AuthNonceConfig, AuthNonceHeader, AuthNonceRepository,
    },
    auth_user::_api::kernel::infra::AuthUserRepository,
};

use super::data::{ConvertPasswordError, PasswordHashError};
use crate::auth::auth_user::_api::kernel::data::AuthUserId;
use crate::auth::login_id::_api::data::LoginId;
use crate::z_details::_api::{message::data::MessageError, repository::data::RepositoryError};

pub trait AuthenticatePasswordInfra {
    type Clock: AuthClock;
    type NonceHeader: AuthNonceHeader;
    type NonceRepository: AuthNonceRepository;
    type PasswordHash: AuthUserPasswordHash;
    type PasswordRepository: AuthUserPasswordRepository;
    type UserRepository: AuthUserRepository;
    type Messenger: AuthenticateMessenger;

    fn nonce_config(&self) -> &AuthNonceConfig;
    fn clock(&self) -> &Self::Clock;
    fn nonce_header(&self) -> &Self::NonceHeader;
    fn nonce_repository(&self) -> &Self::NonceRepository;
    fn password_hash(&self) -> &Self::PasswordHash;
    fn password_repository(&self) -> &Self::PasswordRepository;
    fn user_repository(&self) -> &Self::UserRepository;
    fn messenger(&self) -> &Self::Messenger;
}

pub trait AuthUserPasswordHash {
    fn verify(
        &self,
        plain_password: &PlainPassword,
        hashed_password: &HashedPassword,
    ) -> Result<bool, PasswordHashError>;
}

pub trait AuthUserPasswordRepository {
    fn match_password(
        &self,
        login_id: &LoginId,
        matcher: impl Fn(&HashedPassword) -> Result<bool, PasswordHashError>,
    ) -> Result<Option<AuthUserId>, MatchPasswordError>;
}

pub enum MatchPasswordError {
    PasswordHashError(PasswordHashError),
    RepositoryError(RepositoryError),
}

pub trait AuthenticateMessenger {
    fn decode(&self) -> Result<AuthenticatePasswordFieldsExtract, MessageError>;
    fn encode_invalid_password(&self) -> Result<String, MessageError>;
}

pub struct AuthenticatePasswordFieldsExtract {
    pub login_id: String,
    pub password: String,
}

pub struct HashedPassword(String);

impl HashedPassword {
    pub const fn new(password: String) -> Self {
        Self(password)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

pub struct PlainPassword(String);

impl PlainPassword {
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

// bcrypt を想定しているので、72 バイト以上ではいけない
// ui の設定と同期させること
const PASSWORD_MAX_BYTES: usize = 72;

impl TryInto<PlainPassword> for String {
    type Error = ConvertPasswordError;

    fn try_into(self) -> Result<PlainPassword, Self::Error> {
        match self.chars().count() {
            n if n == 0 => Err(ConvertPasswordError::Empty),
            n if n > PASSWORD_MAX_BYTES => Err(ConvertPasswordError::TooLong),
            _ => Ok(PlainPassword(self)),
        }
    }
}
