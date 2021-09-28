use crate::z_lib::remote::service::data::{ServiceAuthorizeError, ServiceAuthorizeToken};

#[async_trait::async_trait]
pub trait ServiceAuthorizer {
    async fn fetch_token(&self) -> Result<Option<ServiceAuthorizeToken>, ServiceAuthorizeError>;
}
