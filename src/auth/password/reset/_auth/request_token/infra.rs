use crate::auth::{
    auth_ticket::_auth::kernel::infra::{AuthClockInfra, CheckAuthNonceInfra},
    password::_auth::kernel::infra::AuthUserPasswordInfra,
};

use crate::{
    auth::{
        auth_ticket::_auth::kernel::data::{ExpireDateTime, ExpireDuration},
        login_id::_auth::data::LoginId,
        password::{
            _auth::kernel::data::ResetToken,
            reset::_auth::{
                kernel::data::ResetTokenEncoded,
                request_token::data::{
                    EncodeResetTokenError, NotifyResetTokenError, NotifyResetTokenResponse,
                    ResetTokenDestination,
                },
            },
        },
    },
    z_details::_common::repository::data::RepositoryError,
};

pub trait RequestResetTokenInfra {
    type CheckNonceInfra: CheckAuthNonceInfra;
    type ClockInfra: AuthClockInfra;
    type PasswordInfra: AuthUserPasswordInfra;
    type DestinationRepository: ResetTokenDestinationRepository;
    type TokenGenerator: ResetTokenGenerator;
    type TokenEncoder: ResetTokenEncoder;
    type TokenNotifier: ResetTokenNotifier;
    type RequestDecoder: RequestResetTokenRequestDecoder;

    fn extract(
        self,
    ) -> (
        Self::CheckNonceInfra,
        Self::ClockInfra,
        Self::PasswordInfra,
        Self::RequestDecoder,
        Self::DestinationRepository,
        Self::TokenGenerator,
        Self::TokenEncoder,
        Self::TokenNotifier,
        RequestResetTokenConfig,
    );
}

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
