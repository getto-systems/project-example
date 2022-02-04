use crate::z_lib::api::service::data::{ServiceAuthorizeError, ServiceAuthorizeToken};

#[async_trait::async_trait]
pub trait ServiceAuthorizer {
    async fn fetch_token(&self) -> Result<Option<ServiceAuthorizeToken>, ServiceAuthorizeError>;
}
