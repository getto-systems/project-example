use crate::auth::auth_ticket::_common::kernel::infra::{
    AuthServiceMetadata, AuthServiceMetadataContent,
};

use crate::auth::{
    _common::service::data::AuthServiceError,
    auth_user::_common::kernel::data::{AuthUserId, RequireAuthRoles},
};

pub trait ValidateApiTokenInfra {
    type ServiceMetadata: AuthServiceMetadata;
    type ValidateService: ValidateService;

    fn service_metadata(&self) -> &Self::ServiceMetadata;
    fn validate_service(&self) -> &Self::ValidateService;
}

#[async_trait::async_trait]
pub trait ValidateService {
    async fn validate(
        &self,
        metadata: AuthServiceMetadataContent,
        require_roles: RequireAuthRoles,
    ) -> Result<AuthUserId, AuthServiceError>;
}
