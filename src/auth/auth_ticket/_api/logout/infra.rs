use crate::auth::auth_ticket::_common::kernel::infra::{AuthNonceMetadata, AuthTokenMetadata};

use crate::auth::{
    _common::service::data::AuthServiceError,
    auth_ticket::_common::kernel::data::{AuthNonce, AuthToken},
};

pub trait LogoutInfra {
    type NonceMetadata: AuthNonceMetadata;
    type TokenMetadata: AuthTokenMetadata;
    type LogoutService: LogoutService;

    fn nonce_metadata(&self) -> &Self::NonceMetadata;
    fn token_metadata(&self) -> &Self::TokenMetadata;
    fn logout_service(&self) -> &Self::LogoutService;
}

#[async_trait::async_trait]
pub trait LogoutService {
    async fn logout(
        &self,
        nonce: Option<AuthNonce>,
        token: Option<AuthToken>,
    ) -> Result<(), AuthServiceError>;
}
