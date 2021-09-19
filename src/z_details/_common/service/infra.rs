use crate::z_details::_common::service::data::{ServiceAuthorizeError, ServiceAuthorizeToken};

#[async_trait::async_trait]
pub trait ServiceAuthorizer {
    // TODO Option じゃなくできるはず
    async fn fetch_token(&self) -> Result<Option<ServiceAuthorizeToken>, ServiceAuthorizeError>;
}
