use crate::auth::{
    auth_ticket::{
        _auth::kernel::infra::AuthClock, remote::check_nonce::infra::CheckAuthNonceInfra,
    },
    password::{
        remote::kernel::infra::RegisterResetTokenRepository,
        reset::remote::proxy_request_token::infra::RequestResetTokenFieldsExtract,
    },
};

use crate::{
    auth::{
        auth_ticket::_auth::kernel::data::{ExpireDateTime, ExpireDuration},
        login_id::remote::data::LoginId,
        password::{
            remote::kernel::data::ResetToken,
            reset::remote::{
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
    type Clock: AuthClock;
    type PasswordRepository: RegisterResetTokenRepository;
    type DestinationRepository: ResetTokenDestinationRepository;
    type TokenGenerator: ResetTokenGenerator;
    type TokenEncoder: ResetTokenEncoder;
    type TokenNotifier: ResetTokenNotifier;

    fn check_nonce_infra(&self) -> &Self::CheckNonceInfra;
    fn clock(&self) -> &Self::Clock;
    fn password_repository(&self) -> &Self::PasswordRepository;
    fn destination_repository(&self) -> &Self::DestinationRepository;
    fn token_generator(&self) -> &Self::TokenGenerator;
    fn token_encoder(&self) -> &Self::TokenEncoder;
    fn token_notifier(&self) -> &Self::TokenNotifier;
    fn config(&self) -> &RequestResetTokenConfig;
}

pub trait RequestResetTokenRequestDecoder {
    fn decode(self) -> RequestResetTokenFieldsExtract;
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
