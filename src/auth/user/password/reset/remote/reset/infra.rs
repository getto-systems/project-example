use crate::auth::{ticket::remote::{check_nonce::infra::CheckAuthNonceInfra, kernel::infra::AuthClock}, user::{password::{remote::kernel::{data::ResetTokenDestination, infra::{
                AuthUserPasswordHasher, PlainPassword, ResetPasswordRepository,
            }}, reset::remote::{proxy_reset::infra::ResetPasswordFieldsExtract, reset::data::{NotifyResetPasswordError, NotifyResetPasswordResponse}}}, remote::kernel::infra::AuthUserRepository}};

use crate::auth::user::password::reset::remote::reset::event::ResetPasswordEvent;

use crate::auth::user::password::{
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
    type ResetNotifier: ResetPasswordNotifier;

    fn check_nonce_infra(&self) -> &Self::CheckNonceInfra;
    fn clock(&self) -> &Self::Clock;
    fn user_repository(&self) -> &Self::UserRepository;
    fn password_repository(&self) -> &Self::PasswordRepository;
    fn password_hasher(&self, plain_password: PlainPassword) -> Self::PasswordHasher {
        Self::PasswordHasher::new(plain_password)
    }
    fn token_decoder(&self) -> &Self::TokenDecoder;
    fn reset_notifier(&self) -> &Self::ResetNotifier;
}

pub trait ResetTokenDecoder {
    fn decode(&self, token: &ResetTokenEncoded) -> Result<ResetToken, DecodeResetTokenError>;
}

pub trait ResetPasswordRequestDecoder {
    fn decode(self) -> ResetPasswordFieldsExtract;
}

#[async_trait::async_trait]
pub trait ResetPasswordNotifier {
    async fn notify(
        &self,
        destination: ResetTokenDestination,
    ) -> Result<NotifyResetPasswordResponse, NotifyResetPasswordError>;
}

impl Into<ResetPasswordEvent> for ResetPasswordRepositoryError {
    fn into(self) -> ResetPasswordEvent {
        match self {
            Self::RepositoryError(err) => ResetPasswordEvent::RepositoryError(err),
            Self::PasswordHashError(err) => ResetPasswordEvent::PasswordHashError(err),
        }
    }
}
