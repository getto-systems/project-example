use crate::{
    auth::{
        kernel::data::{AuthDateTime, ExpireDateTime, ExpireDuration},
        user::{
            kernel::data::AuthUserId,
            login_id::kernel::data::{LoginId, ValidateLoginIdError},
            password::reset::{
                kernel::data::{
                    ResetPasswordId, ResetPasswordToken, ResetPasswordTokenDestination,
                },
                request_token::data::{
                    EncodeResetTokenError, NotifyResetTokenError, NotifyResetTokenResponse,
                },
            },
        },
    },
    common::api::repository::data::RepositoryError,
};

pub struct RequestResetPasswordTokenFields {
    pub login_id: LoginId,
}

pub trait RequestResetPasswordTokenFieldsExtract {
    fn convert(self) -> Result<RequestResetPasswordTokenFields, ValidateLoginIdError>;
}

#[async_trait::async_trait]
pub trait RegisterResetPasswordTokenRepository {
    async fn lookup_user(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<(AuthUserId, Option<ResetPasswordTokenDestination>)>, RepositoryError>;

    async fn register_reset_token(
        &self,
        reset_token: ResetPasswordId,
        user_id: AuthUserId,
        login_id: LoginId,
        destination: ResetPasswordTokenDestination,
        expires: ExpireDateTime,
        requested_at: AuthDateTime,
    ) -> Result<(), RepositoryError>;
}

pub trait ResetPasswordIdGenerator {
    fn generate(&self) -> ResetPasswordId;
}

pub trait ResetPasswordTokenEncoder {
    fn encode(
        &self,
        token: ResetPasswordId,
        expires: ExpireDateTime,
    ) -> Result<ResetPasswordToken, EncodeResetTokenError>;
}

#[async_trait::async_trait]
pub trait ResetPasswordTokenNotifier {
    async fn notify(
        &self,
        destination: ResetPasswordTokenDestination,
        token: ResetPasswordToken,
    ) -> Result<NotifyResetTokenResponse, NotifyResetTokenError>;
}

pub struct RequestResetPasswordTokenConfig {
    pub token_expires: ExpireDuration,
}
