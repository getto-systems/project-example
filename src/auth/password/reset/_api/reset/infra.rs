pub mod destination_repository;
pub mod messenger;
pub mod token_encoder;
pub mod token_generator;
pub mod token_notifier;

use async_trait::async_trait;

use crate::auth::{
    auth_ticket::_api::kernel::{
        data::{ExpireDateTime, ExpireDuration},
        infra::{AuthClock, CheckAuthNonceInfra},
    },
    password::reset::_api::kernel::{
        data::{ResetToken, ResetTokenEncoded},
        infra::{ResetTokenGenerator, ResetTokenRepository},
    },
};

use super::data::{
    DecodeResetTokenError, NotifyResetTokenError, NotifyResetTokenResponse, ResetTokenDestination,
};
use crate::{
    auth::login_id::_api::data::LoginId,
    z_details::_api::{message::data::MessageError, repository::data::RepositoryError},
};

pub trait RequestResetTokenInfra {
    type CheckNonceInfra: CheckAuthNonceInfra;
    type Clock: AuthClock;
    type DestinationRepository: ResetTokenDestinationRepository;
    type TokenRepository: ResetTokenRepository;
    type TokenGenerator: ResetTokenGenerator;
    type TokenEncoder: ResetTokenEncoder;
    type TokenNotifier: ResetTokenNotifier;
    type Messenger: RequestResetTokenMessenger;

    fn check_nonce_infra(&self) -> &Self::CheckNonceInfra;
    fn config(&self) -> &RequestResetTokenConfig;
    fn clock(&self) -> &Self::Clock;
    fn destination_repository(&self) -> &Self::DestinationRepository;
    fn token_repository(&self) -> &Self::TokenRepository;
    fn token_generator(&self) -> &Self::TokenGenerator;
    fn token_encoder(&self) -> &Self::TokenEncoder;
    fn token_notifier(&self) -> &Self::TokenNotifier;
    fn messenger(&self) -> &Self::Messenger;
}

pub struct RequestResetTokenConfig {
    pub token_expires: ExpireDuration,
}

pub trait ResetTokenDestinationRepository {
    fn get(&self, login_id: &LoginId) -> Result<Option<ResetTokenDestination>, RepositoryError>;
}

pub trait ResetTokenEncoder {
    fn encode(
        &self,
        token: ResetToken,
        expires: ExpireDateTime,
    ) -> Result<ResetTokenEncoded, DecodeResetTokenError>;
}

#[async_trait]
pub trait ResetTokenNotifier {
    async fn notify(
        &self,
        destination: ResetTokenDestination,
        token: &ResetTokenEncoded,
    ) -> Result<NotifyResetTokenResponse, NotifyResetTokenError>;
}

pub trait RequestResetTokenMessenger {
    fn decode(&self) -> Result<RequestResetTokenFieldsExtract, MessageError>;
    fn encode_success(&self) -> Result<String, MessageError>;
    fn encode_invalid_reset(&self) -> Result<String, MessageError>;
}

#[derive(Clone)]
pub struct RequestResetTokenFieldsExtract {
    pub login_id: String,
}
