use tonic::{
    metadata::MetadataValue,
    transport::{Channel, ClientTlsConfig, Endpoint},
    Request,
};
use url::Url;

use crate::x_outside_feature::_common::metadata::METADATA_REQUEST_ID;

use crate::z_details::_common::service::infra::ServiceAuthorizer;

use crate::example::_api::service::data::ExampleServiceError;

pub fn infra_error(err: impl std::fmt::Display) -> ExampleServiceError {
    ExampleServiceError::InfraError(format!("service infra error; {}", err))
}

pub async fn set_authorization<T>(
    request: &mut Request<T>,
    authorizer: &impl ServiceAuthorizer,
) -> Result<(), ExampleServiceError> {
    if let Some(token) = authorizer.fetch_token().await.map_err(infra_error)? {
        request.metadata_mut().insert(
            "authorization",
            MetadataValue::from_str(&format!("Bearer {}", token.extract())).map_err(infra_error)?,
        );
    }
    Ok(())
}

pub fn set_metadata<T>(
    request: &mut Request<T>,
    request_id: &str,
) -> Result<(), ExampleServiceError> {
    request.metadata_mut().append(
        METADATA_REQUEST_ID,
        MetadataValue::from_str(request_id).map_err(infra_error)?,
    );

    Ok(())
}

pub fn new_endpoint(service_url: &'static str) -> Result<Endpoint, ExampleServiceError> {
    let url = Url::parse(service_url).map_err(infra_error)?;
    if url.scheme() == "https" {
        let config = ClientTlsConfig::new().domain_name(url.host_str().ok_or(
            ExampleServiceError::InfraError("invalid service url".into()),
        )?);
        Channel::from_static(service_url)
            .tls_config(config)
            .map_err(infra_error)
    } else {
        Ok(Channel::from_static(service_url))
    }
}
