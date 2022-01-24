use crate::{
    auth::{
        ticket::remote::kernel::data::{ExpireDateTime, ExpireDuration},
        user::{
            login_id::remote::data::LoginId,
            password::{
                remote::kernel::data::{ResetToken, ResetTokenDestination},
                reset::remote::{
                    kernel::data::ResetTokenEncoded,
                    request_token::data::{
                        EncodeResetTokenError, NotifyResetTokenError, NotifyResetTokenResponse,
                    },
                },
            },
        },
    },
    z_lib::remote::repository::data::RepositoryError,
};

pub trait RequestResetTokenRequestDecoder {
    fn decode(self) -> RequestResetTokenFieldsExtract;
}

pub struct RequestResetTokenFieldsExtract {
    pub login_id: String,
}

#[async_trait::async_trait]
pub trait ResetTokenDestinationRepository {
    async fn get(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<ResetTokenDestination>, RepositoryError>;
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
