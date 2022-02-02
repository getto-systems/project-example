use crate::auth::ticket::validate::infra::AuthMetadataContent;

use crate::auth::remote::proxy::data::AuthProxyError;

#[async_trait::async_trait]
pub trait AuthProxyService {
    type Response;

    fn name(&self) -> &str;
    async fn call(self, metadata: AuthMetadataContent) -> Result<Self::Response, AuthProxyError>;
}
