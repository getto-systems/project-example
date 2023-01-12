use tonic::{
    transport::{Channel, ClientTlsConfig},
    Request,
};
use url::Url;

use crate::{
    common::x_outside_feature::feature::CommonOutsideService,
    x_content::metadata::METADATA_REQUEST_ID, x_outside_feature::data::RequestId,
};

use crate::common::api::service::init::authorizer::GoogleServiceAuthorizer;

use crate::common::api::service::infra::ServiceAuthorizer;

use crate::{
    common::api::service::data::{ServiceConnectError, ServiceMetadataError},
    common::proxy::data::ProxyMetadataExtract,
};

pub struct GoogleTonicService<'a> {
    service_url: &'static str,
    request_id: RequestId,
    authorizer: GoogleServiceAuthorizer<'a>,
}

impl<'a> GoogleTonicService<'a> {
    pub fn new(service: &'a CommonOutsideService, request_id: RequestId) -> Self {
        Self {
            service_url: service.service_url,
            request_id,
            authorizer: GoogleServiceAuthorizer::new(&service.google_authorizer),
        }
    }

    pub async fn endpoint(&self) -> Result<Channel, ServiceConnectError> {
        let url = Url::parse(self.service_url).map_err(ServiceConnectError::UrlParseError)?;

        let endpoint = if url.scheme() == "https" {
            let config = ClientTlsConfig::new()
                .domain_name(url.host_str().ok_or(ServiceConnectError::InvalidUrlError)?);
            Channel::from_static(self.service_url)
                .tls_config(config)
                .map_err(ServiceConnectError::TransportError)?
        } else {
            Channel::from_static(self.service_url)
        };

        endpoint
            .connect()
            .await
            .map_err(ServiceConnectError::TransportError)
    }

    pub async fn set_metadata<T>(
        &self,
        request: &mut Request<T>,
        metadata: impl ProxyMetadataExtract,
    ) -> Result<(), ServiceMetadataError> {
        request.metadata_mut().insert(
            METADATA_REQUEST_ID,
            self.request_id
                .clone()
                .extract()
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

        if let Some(token) = self
            .authorizer
            .fetch_token()
            .await
            .map_err(ServiceMetadataError::AuthorizeError)?
        {
            request.metadata_mut().insert(
                "authorization",
                format!("Bearer {}", token.extract())
                    .try_into()
                    .map_err(ServiceMetadataError::InvalidMetadataValue)?,
            );
        }

        Ok(())
    }
}
