use std::fmt::Display;

use tonic::{metadata::MetadataValue, Request};

use crate::{
    auth::auth_ticket::_common::kernel::x_tonic::metadata::{
        METADATA_NONCE, METADATA_TICKET_TOKEN,
    },
    x_outside_feature::_common::metadata::METADATA_REQUEST_ID,
};

use crate::auth::{
    _api::service::data::AuthServiceError,
    auth_ticket::_common::kernel::data::{AuthNonce, AuthToken},
};

pub fn infra_error(err: impl Display) -> AuthServiceError {
    AuthServiceError::InfraError(format!("service infra error; {}", err))
}

pub fn set_metadata<T>(
    request: &mut Request<T>,
    request_id: &str,
    nonce: Option<AuthNonce>,
    token: Option<AuthToken>,
) -> Result<(), AuthServiceError> {
    request.metadata_mut().append(
        METADATA_REQUEST_ID,
        MetadataValue::from_str(request_id).map_err(infra_error)?,
    );
    if let Some(nonce) = nonce {
        request.metadata_mut().append(
            METADATA_NONCE,
            MetadataValue::from_str(&nonce.extract()).map_err(infra_error)?,
        );
    }
    if let Some(token) = token {
        request.metadata_mut().append(
            METADATA_TICKET_TOKEN,
            MetadataValue::from_str(&token.extract()).map_err(infra_error)?,
        );
    }

    Ok(())
}
