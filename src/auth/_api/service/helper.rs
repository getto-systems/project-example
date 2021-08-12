use std::fmt::Display;

use tonic::{metadata::MetadataValue, Request};

use crate::{
    auth::auth_ticket::_common::kernel::x_tonic::metadata::{
        METADATA_NONCE, METADATA_TICKET_TOKEN,
    },
    x_outside_feature::_common::metadata::METADATA_REQUEST_ID,
};

use crate::auth::{
    _api::service::data::ServiceError,
    auth_ticket::_common::kernel::data::{AuthNonceValue, AuthTokenValue},
};

pub fn infra_error(err: impl Display) -> ServiceError {
    ServiceError::InfraError(format!("service infra error; {}", err))
}

pub fn set_metadata<T>(
    request: &mut Request<T>,
    request_id: &str,
    nonce: AuthNonceValue,
    token: AuthTokenValue,
) -> Result<(), ServiceError> {
    request.metadata_mut().append(
        METADATA_REQUEST_ID,
        MetadataValue::from_str(request_id).map_err(infra_error)?,
    );
    request.metadata_mut().append(
        METADATA_NONCE,
        MetadataValue::from_str(&nonce.extract()).map_err(infra_error)?,
    );
    request.metadata_mut().append(
        METADATA_TICKET_TOKEN,
        MetadataValue::from_str(&token.extract()).map_err(infra_error)?,
    );

    Ok(())
}
