use crate::auth::auth_ticket::_api::kernel::infra::{AuthNonceHeader, AuthTokenHeader};

use crate::auth::{
    _api::service::data::AuthServiceError,
    auth_ticket::_common::kernel::data::{AuthNonce, AuthToken},
    auth_user::_common::kernel::data::{AuthUserId, RequireAuthRoles},
};

pub trait ValidateInfra {
    type NonceHeader: AuthNonceHeader;
    type TokenHeader: AuthTokenHeader;
    type ValidateService: ValidateService;

    fn nonce_header(&self) -> &Self::NonceHeader;
    fn token_header(&self) -> &Self::TokenHeader;
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
