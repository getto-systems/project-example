use crate::auth::{
    kernel::infra::AuthClock,
    user::password::kernel::infra::{AuthUserPasswordHasher, HashedPassword, PlainPassword},
};

use crate::{
    auth::{
        kernel::data::{AuthDateTime, ExpireDateTime},
        ticket::kernel::data::{AuthPermissionGranted, AuthenticateSuccess},
        user::{
            kernel::data::AuthUserId,
            password::{
                kernel::data::PasswordHashError,
                reset::{
                    kernel::data::{
                        ResetPasswordId, ResetPasswordToken, ResetPasswordTokenDestination,
                    },
                    reset::data::{
                        DecodeResetTokenError, NotifyResetPasswordError,
                        NotifyResetPasswordResponse, ResetPasswordError,
                        ValidateResetPasswordFieldsError,
                    },
                },
            },
        },
    },
    common::api::repository::data::RepositoryError,
};

pub struct ResetPasswordFields {
    pub reset_token: ResetPasswordToken,
    pub new_password: PlainPassword,
}

pub trait ResetPasswordFieldsExtract {
    fn convert(self) -> Result<ResetPasswordFields, ValidateResetPasswordFieldsError>;
}

impl ResetPasswordFieldsExtract for ResetPasswordFields {
    fn convert(self) -> Result<ResetPasswordFields, ValidateResetPasswordFieldsError> {
        Ok(self)
    }
}

pub trait ResetPasswordInfra {
    type Clock: AuthClock;
    type Repository: ResetPasswordRepository;
    type PasswordHasher: AuthUserPasswordHasher;
    type TokenDecoder: ResetPasswordTokenDecoder;
    type ResetNotifier: ResetPasswordNotifier;

    fn clock(&self) -> &Self::Clock;
    fn repository(&self) -> &Self::Repository;
    fn password_hasher(&self, plain_password: PlainPassword) -> Self::PasswordHasher {
        Self::PasswordHasher::new(plain_password)
    }
    fn token_decoder(&self) -> &Self::TokenDecoder;
    fn reset_notifier(&self) -> &Self::ResetNotifier;
}

pub trait ResetPasswordTokenDecoder {
    fn decode(&self, token: ResetPasswordToken) -> Result<ResetPasswordId, DecodeResetTokenError>;
}

#[async_trait::async_trait]
pub trait ResetPasswordRepository {
    async fn lookup_reset_token_entry(
        &self,
        reset_id: &ResetPasswordId,
    ) -> Result<
        Option<(
            AuthUserId,
            ResetPasswordTokenDestination,
            ResetPasswordTokenMoment,
        )>,
        RepositoryError,
    >;

    async fn lookup_permission_granted(
        &self,
        user_id: &AuthUserId,
    ) -> Result<Option<AuthPermissionGranted>, RepositoryError>;

    async fn consume_reset_id(
        &self,
        reset_id: ResetPasswordId,
        reset_at: AuthDateTime,
    ) -> Result<(), RepositoryError>;
    async fn update_password(
        &self,
        user_id: AuthUserId,
        new_password: HashedPassword,
    ) -> Result<(), RepositoryError>;
}

pub struct ResetPasswordTokenMoment {
    expires: ExpireDateTime,
    reset_at: Option<AuthDateTime>,
}

impl ResetPasswordTokenMoment {
    pub(in crate::auth) const fn restore(
        expires: ExpireDateTime,
        reset_at: Option<AuthDateTime>,
    ) -> Self {
        Self { expires, reset_at }
    }

    pub fn has_expired(&self, now: &AuthDateTime) -> bool {
        self.expires.has_elapsed(now)
    }

    pub fn has_already_reset(&self) -> bool {
        self.reset_at.is_some()
    }
}

#[async_trait::async_trait]
pub trait ResetPasswordNotifier {
    async fn notify(
        &self,
        destination: ResetPasswordTokenDestination,
    ) -> Result<NotifyResetPasswordResponse, NotifyResetPasswordError>;
}

pub trait ResetPasswordLogger: Send + Sync {
    fn try_to_reset_password(&self);
    fn invalid_request(
        &self,
        err: ValidateResetPasswordFieldsError,
    ) -> ValidateResetPasswordFieldsError;
    fn failed_to_decode_token(&self, err: DecodeResetTokenError) -> DecodeResetTokenError;
    fn failed_to_lookup_reset_token_entry(&self, err: RepositoryError) -> RepositoryError;
    fn reset_token_not_found(&self, err: ResetPasswordError) -> ResetPasswordError;
    fn failed_to_lookup_permission_granted(&self, err: RepositoryError) -> RepositoryError;
    fn already_reset(&self, err: ResetPasswordError) -> ResetPasswordError;
    fn expired(&self, err: ResetPasswordError) -> ResetPasswordError;
    fn failed_to_hash_password(&self, err: PasswordHashError) -> PasswordHashError;
    fn failed_to_consume_reset_id(&self, err: RepositoryError) -> RepositoryError;
    fn failed_to_update_password(&self, err: RepositoryError) -> RepositoryError;
    fn failed_to_notify(&self, err: NotifyResetPasswordError) -> NotifyResetPasswordError;
    fn succeed_to_notify(
        &self,
        response: NotifyResetPasswordResponse,
    ) -> NotifyResetPasswordResponse;
    fn succeed_to_reset_password(&self, auth: AuthenticateSuccess) -> AuthenticateSuccess;
}
