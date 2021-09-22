use tonic::{
    metadata::MetadataValue,
    transport::{Channel, ClientTlsConfig, Endpoint},
    Request,
};
use url::Url;

use crate::{
    auth::_common::metadata::{METADATA_NONCE, METADATA_TOKEN},
    x_outside_feature::_common::metadata::METADATA_REQUEST_ID,
};

use crate::{
    auth::auth_ticket::_common::kernel::infra::AuthMetadataContent,
    z_details::_common::service::infra::ServiceAuthorizer,
};

use crate::auth::_common::service::data::AuthServiceError;

pub fn infra_error(err: impl std::fmt::Display) -> AuthServiceError {
    AuthServiceError::InfraError(format!("service infra error; {}", err))
}

pub fn new_endpoint(service_url: &'static str) -> Result<Endpoint, AuthServiceError> {
    let url = Url::parse(service_url).map_err(infra_error)?;
    if url.scheme() == "https" {
        let config = ClientTlsConfig::new().domain_name(
            url.host_str()
                .ok_or(AuthServiceError::InfraError("invalid service url".into()))?,
        );
        Channel::from_static(service_url)
            .tls_config(config)
            .map_err(infra_error)
    } else {
        Ok(Channel::from_static(service_url))
    }
}

pub async fn set_authorization<T>(
    request: &mut Request<T>,
    authorizer: &impl ServiceAuthorizer,
) -> Result<(), AuthServiceError> {
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
    metadata: AuthMetadataContent,
) -> Result<(), AuthServiceError> {
    request.metadata_mut().insert(
        METADATA_REQUEST_ID,
        MetadataValue::from_str(request_id).map_err(infra_error)?,
    );
    if let Some(nonce) = metadata.nonce {
        request.metadata_mut().insert(
            METADATA_NONCE,
            MetadataValue::from_str(&nonce.extract()).map_err(infra_error)?,
        );
    }
    if let Some(token) = metadata.token {
        request.metadata_mut().insert(
            METADATA_TOKEN,
            MetadataValue::from_str(&token.extract()).map_err(infra_error)?,
        );
    }

    Ok(())
}
