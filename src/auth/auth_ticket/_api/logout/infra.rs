use crate::auth::auth_ticket::_api::kernel::infra::{AuthNonceHeader, AuthTokenHeader};

use crate::auth::{
    _api::service::data::AuthServiceError,
    auth_ticket::_common::kernel::data::{AuthNonceValue, AuthTokenValue},
};

pub trait LogoutInfra {
    type NonceHeader: AuthNonceHeader;
    type TokenHeader: AuthTokenHeader;
    type LogoutService: LogoutService;

    fn nonce_header(&self) -> &Self::NonceHeader;
    fn token_header(&self) -> &Self::TokenHeader;
    fn logout_service(&self) -> &Self::LogoutService;
}

#[async_trait::async_trait]
pub trait LogoutService {
    async fn logout(
        &self,
        nonce: Option<AuthNonceValue>,
        token: Option<AuthTokenValue>,
    ) -> Result<(), AuthServiceError>;
}
