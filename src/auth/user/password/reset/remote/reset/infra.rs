use crate::auth::user::password::{
    kernel::infra::AuthUserPasswordHasher, reset::remote::kernel::infra::ResetTokenEntry,
};

use crate::{
    auth::{
        ticket::kernel::remote::data::AuthDateTime,
        user::{
            password::reset::remote::{
                kernel::data::{ResetToken, ResetTokenDestination, ResetTokenEncoded},
                reset::data::{
                    DecodeResetTokenError, NotifyResetPasswordError, NotifyResetPasswordResponse,
                    ResetPasswordRepositoryError,
                },
            },
            remote::kernel::data::AuthUserId,
        },
    },
    z_lib::remote::repository::data::RepositoryError,
};

pub trait ResetTokenDecoder {
    fn decode(&self, token: &ResetTokenEncoded) -> Result<ResetToken, DecodeResetTokenError>;
}

pub trait ResetPasswordRequestDecoder {
    fn decode(self) -> ResetPasswordFieldsExtract;
}

pub struct ResetPasswordFieldsExtract {
    pub reset_token: String,
    pub login_id: String,
    pub password: String,
}

#[async_trait::async_trait]
pub trait ResetPasswordRepository {
    async fn reset_token_entry(
        &self,
        reset_token: &ResetToken,
    ) -> Result<Option<ResetTokenEntry>, RepositoryError>;

    async fn reset_password<'a>(
        &self,
        reset_token: &'a ResetToken,
        hasher: impl AuthUserPasswordHasher + 'a,
        reset_at: AuthDateTime,
    ) -> Result<AuthUserId, ResetPasswordRepositoryError>;
}

#[async_trait::async_trait]
pub trait ResetPasswordNotifier {
    async fn notify(
        &self,
        destination: ResetTokenDestination,
    ) -> Result<NotifyResetPasswordResponse, NotifyResetPasswordError>;
}
