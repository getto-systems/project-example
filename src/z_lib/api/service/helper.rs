use tonic::transport::{Channel, ClientTlsConfig, Endpoint};
use url::Url;

use crate::z_lib::service::data::ServiceEndpointError;

pub fn new_endpoint(service_url: &'static str) -> Result<Endpoint, ServiceEndpointError> {
    let url = Url::parse(service_url).map_err(ServiceEndpointError::ParseError)?;
    if url.scheme() == "https" {
        let config = ClientTlsConfig::new().domain_name(
            url.host_str()
                .ok_or(ServiceEndpointError::InvalidUrlError)?,
        );
        Channel::from_static(service_url)
            .tls_config(config)
            .map_err(ServiceEndpointError::InvalidTlsError)
    } else {
        Ok(Channel::from_static(service_url))
    }
}
