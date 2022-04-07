use crate::{
    auth::{
        ticket::kernel::data::{AuthDateTime, ExpireDateTime, ExpireDuration},
        user::{
            kernel::data::AuthUserId,
            login_id::kernel::data::LoginId,
            password::reset::{
                kernel::data::{ResetToken, ResetTokenDestination, ResetTokenEncoded},
                request_token::data::{
                    EncodeResetTokenError, NotifyResetTokenError, NotifyResetTokenResponse,
                },
            },
        },
    },
    z_lib::repository::data::RepositoryError,
};

pub trait RequestResetTokenRequestDecoder {
    fn decode(self) -> RequestResetTokenFieldsExtract;
}

pub struct RequestResetTokenFields {
    pub login_id: LoginId,
}

pub struct RequestResetTokenFieldsExtract {
    pub login_id: String,
}

#[async_trait::async_trait]
pub trait RegisterResetTokenRepository {
    async fn lookup_user(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<(AuthUserId, Option<ResetTokenDestination>)>, RepositoryError>;

    async fn register_reset_token(
        &self,
        reset_token: ResetToken,
        user_id: AuthUserId,
        login_id: LoginId,
        destination: ResetTokenDestination,
        expires: ExpireDateTime,
        requested_at: AuthDateTime,
    ) -> Result<(), RepositoryError>;
}

pub trait ResetTokenGenerator {
    fn generate(&self) -> ResetToken;
}

pub trait ResetTokenEncoder {
    fn encode(
        &self,
        token: ResetToken,
        expires: ExpireDateTime,
    ) -> Result<ResetTokenEncoded, EncodeResetTokenError>;
}

#[async_trait::async_trait]
pub trait ResetTokenNotifier {
    async fn notify(
        &self,
        destination: ResetTokenDestination,
        token: ResetTokenEncoded,
    ) -> Result<NotifyResetTokenResponse, NotifyResetTokenError>;
}

pub struct RequestResetTokenConfig {
    pub token_expires: ExpireDuration,
}
