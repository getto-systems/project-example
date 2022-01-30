use crate::auth::ticket::remote::validate::infra::AuthMetadataContent;

#[async_trait::async_trait]
pub trait AuthProxyService {
    type Response;
    type Error;

    fn name(&self) -> &str;
    async fn call(self, metadata: AuthMetadataContent) -> Result<Self::Response, Self::Error>;
}
