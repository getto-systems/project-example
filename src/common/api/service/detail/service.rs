use tonic::{
    transport::{Channel, ClientTlsConfig},
    Request,
};
use url::Url;

use crate::{
    common::api::service::data::ServiceAuthorizeToken,
    x_content::metadata::{METADATA_METHOD, METADATA_PATH, METADATA_REQUEST_ID},
};

use crate::common::{
    api::{
        request::data::RequestInfo,
        service::data::{ServiceConnectError, ServiceMetadataError},
    },
    proxy::data::ProxyMetadataExtract,
};

pub struct TonicService;

impl TonicService {
    pub async fn endpoint(service_url: &'static str) -> Result<Channel, ServiceConnectError> {
        let url = Url::parse(service_url).map_err(ServiceConnectError::UrlParseError)?;

        let endpoint = if url.scheme() == "https" {
            let config = ClientTlsConfig::new()
                .domain_name(url.host_str().ok_or(ServiceConnectError::InvalidUrlError)?);
            Channel::from_static(service_url)
                .tls_config(config)
                .map_err(ServiceConnectError::TransportError)?
        } else {
            Channel::from_static(service_url)
        };

        endpoint
            .connect()
            .await
            .map_err(ServiceConnectError::TransportError)
    }

    pub fn set_metadata<T>(
        request: &mut Request<T>,
        authorize_token: Option<ServiceAuthorizeToken>,
        info: RequestInfo,
        metadata: impl ProxyMetadataExtract,
    ) -> Result<(), ServiceMetadataError> {
        if let Some(token) = authorize_token {
            request.metadata_mut().insert(
                "authorization",
                format!("Bearer {}", token.extract())
                    .try_into()
                    .map_err(ServiceMetadataError::InvalidMetadataValue)?,
            );
        }

        let info = info.extract();

        request.metadata_mut().insert(
            METADATA_REQUEST_ID,
            info.id
                .try_into()
                .map_err(ServiceMetadataError::InvalidMetadataValue)?,
        );
        request.metadata_mut().insert(
            METADATA_PATH,
            info.path
                .try_into()
                .map_err(ServiceMetadataError::InvalidMetadataValue)?,
        );
        request.metadata_mut().insert(
            METADATA_METHOD,
            info.method
                .try_into()
                .map_err(ServiceMetadataError::InvalidMetadataValue)?,
        );

        let metadata = metadata
            .convert()
            .map_err(ServiceMetadataError::MetadataError)?;

        for (name, value) in metadata {
            request.metadata_mut().insert(
                name,
                value
                    .try_into()
                    .map_err(ServiceMetadataError::InvalidMetadataValue)?,
            );
        }

        Ok(())
    }
}
