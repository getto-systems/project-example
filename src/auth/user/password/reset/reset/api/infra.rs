use crate::auth::user::password::kernel::infra::{HashedPassword, PlainPassword};

use crate::{
    auth::{
        kernel::data::{AuthDateTime, ExpireDateTime},
        ticket::kernel::data::AuthPermissionGranted,
        user::{
            kernel::data::AuthUserId,
            login_id::kernel::data::LoginId,
            password::reset::{
                kernel::data::{
                    ResetPasswordId, ResetPasswordToken, ResetPasswordTokenDestination,
                },
                reset::data::{
                    DecodeResetTokenError, NotifyResetPasswordError, NotifyResetPasswordResponse,
                    ValidateResetPasswordFieldsError,
                },
            },
        },
    },
    common::api::repository::data::RepositoryError,
};

pub struct ResetPasswordFields {
    pub reset_token: ResetPasswordToken,
    pub login_id: LoginId,
    pub new_password: PlainPassword,
}

pub trait ResetPasswordFieldsExtract {
    fn convert(self) -> Result<ResetPasswordFields, ValidateResetPasswordFieldsError>;
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
            LoginId,
            ResetPasswordTokenDestination,
            ResetPasswordTokenMoment,
        )>,
        RepositoryError,
    >;

    async fn lookup_permission_granted(
        &self,
        user_id: &AuthUserId,
    ) -> Result<Option<AuthPermissionGranted>, RepositoryError>;

    async fn reset_password(
        &self,
        user_id: AuthUserId,
        reset_id: ResetPasswordId,
        new_password: HashedPassword,
        reset_at: AuthDateTime,
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
