use crate::auth::ticket::remote::kernel::infra::AuthMetadataContent;

use crate::auth::{
    remote::service::data::AuthServiceError,
    user::remote::kernel::data::{AuthUserId, RequireAuthRoles},
};

pub trait ValidateApiTokenRequestDecoder {
    fn decode(self) -> RequireAuthRoles;
}

#[async_trait::async_trait]
pub trait ValidateService {
    async fn validate(
        &self,
        metadata: AuthMetadataContent,
        require_roles: RequireAuthRoles,
    ) -> Result<AuthUserId, AuthServiceError>;
}
