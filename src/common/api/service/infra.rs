use crate::common::api::service::data::{ServiceAuthorizeError, ServiceAuthorizeToken};

#[async_trait::async_trait]
pub trait ServiceAuthorizer: Send + Sync {
    async fn fetch_token(
        &self,
        service_url: &str,
    ) -> Result<Option<ServiceAuthorizeToken>, ServiceAuthorizeError>;
}
