use crate::auth::{
    auth_ticket::_api::kernel::infra::CheckAuthNonceInfra,
    password::_api::kernel::infra::{AuthUserPasswordInfra, RequestResetTokenError},
};

use crate::auth::password::reset::_api::request_token::event::RequestResetTokenEvent;

use crate::auth::{
    auth_ticket::_api::kernel::data::{ExpireDateTime, ExpireDuration},
    login_id::_api::data::LoginId,
    password::{
        _api::kernel::data::ResetToken,
        reset::_api::{
            kernel::data::ResetTokenEncoded,
            request_token::data::{
                EncodeResetTokenError, NotifyResetTokenError, NotifyResetTokenResponse,
                RequestResetTokenResponse, ResetTokenDestination,
            },
        },
    },
};
use crate::z_details::{
    _api::message::data::MessageError, _common::repository::data::RepositoryError,
};

pub trait RequestResetTokenInfra {
    type CheckNonceInfra: CheckAuthNonceInfra;
    type PasswordInfra: AuthUserPasswordInfra;
    type DestinationRepository: ResetTokenDestinationRepository;
    type TokenGenerator: ResetTokenGenerator;
    type TokenEncoder: ResetTokenEncoder;
    type TokenNotifier: ResetTokenNotifier;
    type Messenger: RequestResetTokenMessenger;

    fn check_nonce_infra(&self) -> &Self::CheckNonceInfra;
    fn password_infra(&self) -> &Self::PasswordInfra;
    fn destination_repository(&self) -> &Self::DestinationRepository;
    fn token_generator(&self) -> &Self::TokenGenerator;
    fn token_encoder(&self) -> &Self::TokenEncoder;
    fn token_notifier(&self) -> &Self::TokenNotifier;
    fn messenger(&self) -> &Self::Messenger;
    fn config(&self) -> &RequestResetTokenConfig;
}

pub struct RequestResetTokenConfig {
    pub token_expires: ExpireDuration,
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

pub trait RequestResetTokenMessenger {
    fn decode(&self) -> Result<RequestResetTokenFieldsExtract, MessageError>;
    fn encode_success(&self) -> Result<RequestResetTokenResponse, MessageError>;
    fn encode_destination_not_found(&self) -> Result<RequestResetTokenResponse, MessageError>;
    fn encode_user_not_found(&self) -> Result<RequestResetTokenResponse, MessageError>;
}

#[derive(Clone)]
pub struct RequestResetTokenFieldsExtract {
    pub login_id: String,
}

impl RequestResetTokenError {
    pub fn into_request_reset_token_event(
        self,
        messenger: &impl RequestResetTokenMessenger,
    ) -> RequestResetTokenEvent {
        match self {
            Self::RepositoryError(err) => RequestResetTokenEvent::RepositoryError(err),
            Self::NotFound => messenger.encode_user_not_found().into(),
        }
    }
}

impl Into<RequestResetTokenEvent> for Result<RequestResetTokenResponse, MessageError> {
    fn into(self) -> RequestResetTokenEvent {
        match self {
            Ok(response) => RequestResetTokenEvent::InvalidReset(response),
            Err(err) => RequestResetTokenEvent::MessageError(err),
        }
    }
}
