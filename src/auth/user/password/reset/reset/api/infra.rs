use crate::auth::user::password::kernel::infra::{HashedPassword, PlainPassword};

use crate::{
    auth::{
        ticket::kernel::data::{AuthDateTime, ExpireDateTime},
        user::{
            kernel::data::{AuthUserId, GrantedAuthRoles},
            login_id::kernel::data::LoginId,
            password::reset::{
                kernel::data::{ResetToken, ResetTokenDestination, ResetTokenEncoded},
                reset::data::{
                    DecodeResetTokenError, NotifyResetPasswordError, NotifyResetPasswordResponse,
                },
            },
        },
    },
    z_lib::repository::data::RepositoryError,
};

pub struct ResetPasswordFields {
    pub reset_token: ResetTokenEncoded,
    pub login_id: LoginId,
    pub new_password: PlainPassword,
}

pub struct ResetPasswordFieldsExtract {
    pub reset_token: String,
    pub login_id: String,
    pub new_password: String,
}

pub trait ResetTokenDecoder {
    fn decode(&self, token: ResetTokenEncoded) -> Result<ResetToken, DecodeResetTokenError>;
}

pub trait ResetPasswordRequestDecoder {
    fn decode(self) -> ResetPasswordFieldsExtract;
}

#[async_trait::async_trait]
pub trait ResetPasswordRepository {
    async fn lookup_reset_token_entry(
        &self,
        reset_token: &ResetToken,
    ) -> Result<
        Option<(AuthUserId, LoginId, ResetTokenDestination, ResetTokenMoment)>,
        RepositoryError,
    >;

    async fn lookup_granted_roles<'a>(
        &self,
        user_id: &'a AuthUserId,
    ) -> Result<Option<GrantedAuthRoles>, RepositoryError>;

    async fn reset_password(
        &self,
        reset_token: &ResetToken,
        user_id: &AuthUserId,
        new_password: HashedPassword,
        reset_at: AuthDateTime,
    ) -> Result<(), RepositoryError>;
}

pub struct ResetTokenMoment {
    expires: ExpireDateTime,
    reset_at: Option<AuthDateTime>,
}

impl ResetTokenMoment {
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
        destination: ResetTokenDestination,
    ) -> Result<NotifyResetPasswordResponse, NotifyResetPasswordError>;
}
