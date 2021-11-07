use tonic::{
    metadata::{errors::InvalidMetadataValue, MetadataValue},
    Request,
};

use crate::{
    auth::ticket::remote::kernel::x_tonic::metadata::{METADATA_NONCE, METADATA_TOKEN},
    x_outside_feature::remote::common::metadata::METADATA_REQUEST_ID,
};

use crate::auth::ticket::remote::kernel::infra::AuthMetadataContent;

use crate::{
    auth::remote::service::data::AuthServiceError,
    z_lib::remote::service::data::ServiceAuthorizeToken,
};

pub fn infra_error(err: impl std::fmt::Display) -> AuthServiceError {
    AuthServiceError::InfraError(format!("service infra error; {}", err))
}

pub fn set_metadata<T>(
    request: &mut Request<T>,
    request_id: &str,
    token: Option<ServiceAuthorizeToken>,
    metadata: AuthMetadataContent,
) -> Result<(), InvalidMetadataValue> {
    request
        .metadata_mut()
        .insert(METADATA_REQUEST_ID, MetadataValue::from_str(request_id)?);

    // TODO None だったらエラーにしたい
    if let Some(token) = token {
        request.metadata_mut().insert(
            "authorization",
            MetadataValue::from_str(&format!("Bearer {}", token.extract()))?,
        );
    }

    // TODO None だったらエラーにしたい
    if let Some(nonce) = metadata.nonce {
        request
            .metadata_mut()
            .insert(METADATA_NONCE, MetadataValue::from_str(&nonce.extract())?);
    }

    // token は None のことがある
    if let Some(token) = metadata.token {
        request
            .metadata_mut()
            .insert(METADATA_TOKEN, MetadataValue::from_str(&token.extract())?);
    }

    Ok(())
}
