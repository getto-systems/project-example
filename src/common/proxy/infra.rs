use crate::common::proxy::data::ProxyMetadataExtract;

#[async_trait::async_trait]
pub trait ProxyCall {
    type Request;
    type Response;
    type Error;

    async fn call(
        &self,
        metadata: impl ProxyMetadataExtract,
        request: Self::Request,
    ) -> Result<Self::Response, Self::Error>;
}
