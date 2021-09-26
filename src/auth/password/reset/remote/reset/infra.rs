use crate::auth::{
    auth_ticket::_auth::kernel::infra::{AuthClock, CheckAuthNonceInfra},
    auth_user::remote::kernel::infra::AuthUserRepository,
    password::{
        remote::kernel::infra::{AuthUserPasswordHasher, ResetPasswordRepository, PlainPassword},
        reset::remote::proxy_reset::infra::ResetPasswordFieldsExtract,
    },
};

use crate::auth::password::reset::remote::reset::event::ResetPasswordEvent;

use crate::auth::password::{
    remote::kernel::data::{ResetPasswordRepositoryError, ResetToken},
    reset::remote::{kernel::data::ResetTokenEncoded, reset::data::DecodeResetTokenError},
};

pub trait ResetPasswordInfra {
    type CheckNonceInfra: CheckAuthNonceInfra;
    type Clock: AuthClock;
    type UserRepository: AuthUserRepository;
    type PasswordRepository: ResetPasswordRepository;
    type PasswordHasher: AuthUserPasswordHasher;
    type TokenDecoder: ResetTokenDecoder;

    fn check_nonce_infra(&self) -> &Self::CheckNonceInfra;
    fn clock(&self) -> &Self::Clock;
    fn user_repository(&self) -> &Self::UserRepository;
    fn password_repository(&self) -> &Self::PasswordRepository;
    fn password_hasher(&self, plain_password: PlainPassword) -> Self::PasswordHasher {
        Self::PasswordHasher::new(plain_password)
    }
    fn token_decoder(&self) -> &Self::TokenDecoder;
}

pub trait ResetTokenDecoder {
    fn decode(&self, token: &ResetTokenEncoded) -> Result<ResetToken, DecodeResetTokenError>;
}

pub trait ResetPasswordRequestDecoder {
    fn decode(self) -> ResetPasswordFieldsExtract;
}

impl Into<ResetPasswordEvent> for ResetPasswordRepositoryError {
    fn into(self) -> ResetPasswordEvent {
        match self {
            Self::RepositoryError(err) => ResetPasswordEvent::RepositoryError(err),
            Self::PasswordHashError(err) => ResetPasswordEvent::PasswordHashError(err),
        }
    }
}
