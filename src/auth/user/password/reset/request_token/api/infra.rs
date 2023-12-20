use crate::auth::kernel::infra::AuthClock;

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
                    RequestResetPasswordTokenError,
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

impl RequestResetPasswordTokenFieldsExtract for RequestResetPasswordTokenFields {
    fn convert(self) -> Result<RequestResetPasswordTokenFields, ValidateLoginIdError> {
        Ok(self)
    }
}

pub trait RequestResetPasswordTokenInfra {
    type Clock: AuthClock;
    type Repository: RequestResetPasswordTokenRepository;
    type IdGenerator: ResetPasswordIdGenerator;
    type TokenEncoder: ResetPasswordTokenEncoder;
    type TokenNotifier: ResetPasswordTokenNotifier;

    fn clock(&self) -> &Self::Clock;
    fn repository(&self) -> &Self::Repository;
    fn id_generator(&self) -> &Self::IdGenerator;
    fn token_encoder(&self) -> &Self::TokenEncoder;
    fn token_notifier(&self) -> &Self::TokenNotifier;
    fn config(&self) -> &RequestResetPasswordTokenConfig;
}

#[async_trait::async_trait]
pub trait RequestResetPasswordTokenRepository {
    async fn lookup_user(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<(AuthUserId, ResetPasswordTokenDestination)>, RepositoryError>;

    async fn register_reset_token(
        &self,
        reset_token: ResetPasswordId,
        user_id: AuthUserId,
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

#[derive(Clone)]
pub struct RequestResetPasswordTokenConfig {
    pub token_expires: ExpireDuration,
}

pub trait RequestResetPasswordTokenLogger: Send + Sync {
    fn try_to_request_reset_password_token(&self);
    fn invalid_request(&self, err: ValidateLoginIdError) -> ValidateLoginIdError;
    fn failed_to_lookup_user(&self, err: RepositoryError) -> RepositoryError;
    fn user_not_found(&self, err: RequestResetPasswordTokenError)
        -> RequestResetPasswordTokenError;
    fn calculate_token_expires(&self, expires: ExpireDateTime) -> ExpireDateTime;
    fn failed_to_register_reset_token(&self, err: RepositoryError) -> RepositoryError;
    fn failed_to_encode_reset_token(&self, err: EncodeResetTokenError) -> EncodeResetTokenError;
    fn failed_to_notify_reset_token(&self, err: NotifyResetTokenError) -> NotifyResetTokenError;
    fn succeed_to_request_reset_password_token(
        &self,
        response: NotifyResetTokenResponse,
    ) -> NotifyResetTokenResponse;
}
