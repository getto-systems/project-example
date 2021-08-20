use tonic::{metadata::MetadataValue, Request};

use crate::x_outside_feature::_common::metadata::METADATA_REQUEST_ID;

use crate::outline::_api::service::data::OutlineServiceError;

pub fn infra_error(err: impl std::fmt::Display) -> OutlineServiceError {
    OutlineServiceError::InfraError(format!("service infra error; {}", err))
}

pub fn set_metadata<T>(
    request: &mut Request<T>,
    request_id: &str,
) -> Result<(), OutlineServiceError> {
    request.metadata_mut().append(
        METADATA_REQUEST_ID,
        MetadataValue::from_str(request_id).map_err(infra_error)?,
    );

    Ok(())
}
