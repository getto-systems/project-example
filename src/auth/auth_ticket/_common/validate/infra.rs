use crate::auth::_common::infra::AuthTokenDecoder;
use crate::auth::auth_ticket::_common::kernel::infra::{
    AuthMetadata, AuthMetadataContent,
};

use crate::auth::{
    _common::service::data::AuthServiceError,
    auth_user::_common::kernel::data::{AuthUserId, RequireAuthRoles},
};

pub trait ValidateApiTokenInfra {
    type AuthMetadata: AuthMetadata;
    type TokenDecoder: AuthTokenDecoder;
    type ValidateService: ValidateService;

    fn auth_metadata(&self) -> &Self::AuthMetadata;
    fn token_decoder(&self) -> &Self::TokenDecoder;
    fn validate_service(&self) -> &Self::ValidateService;
}

#[async_trait::async_trait]
pub trait ValidateService {
    async fn validate(
        &self,
        metadata: AuthMetadataContent,
        require_roles: RequireAuthRoles,
    ) -> Result<AuthUserId, AuthServiceError>;
}
