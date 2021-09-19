use crate::auth::auth_ticket::_common::kernel::infra::{AuthNonceMetadata, AuthTokenMetadata};

use crate::auth::{
    _common::service::data::AuthServiceError,
    auth_ticket::_common::kernel::data::{AuthNonce, AuthToken},
    auth_user::_common::kernel::data::{AuthUserId, RequireAuthRoles},
};

pub trait ValidateApiTokenInfra {
    type NonceMetadata: AuthNonceMetadata;
    type TokenMetadata: AuthTokenMetadata;
    type ValidateService: ValidateService;

    fn nonce_metadata(&self) -> &Self::NonceMetadata;
    fn token_metadata(&self) -> &Self::TokenMetadata;
    fn validate_service(&self) -> &Self::ValidateService;
}

#[async_trait::async_trait]
pub trait ValidateService {
    async fn validate(
        &self,
        nonce: Option<AuthNonce>,
        token: Option<AuthToken>,
        require_roles: RequireAuthRoles,
    ) -> Result<AuthUserId, AuthServiceError>;
}
