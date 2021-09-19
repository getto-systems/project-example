use crate::auth::auth_ticket::_common::kernel::infra::{
    AuthServiceMetadata, AuthServiceMetadataContent,
};

use crate::auth::_common::service::data::AuthServiceError;

pub trait LogoutInfra {
    type ServiceMetadata: AuthServiceMetadata;
    type LogoutService: LogoutService;

    fn service_metadata(&self) -> &Self::ServiceMetadata;
    fn logout_service(&self) -> &Self::LogoutService;
}

#[async_trait::async_trait]
pub trait LogoutService {
    async fn logout(&self, metadata: AuthServiceMetadataContent) -> Result<(), AuthServiceError>;
}
