use crate::auth::ticket::remote::kernel::infra::{
    AuthMetadata, AuthMetadataContent, AuthTokenDecoder,
};

use crate::auth::{
    _common::service::data::AuthServiceError,
    user::remote::kernel::data::{AuthUserId, RequireAuthRoles},
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
