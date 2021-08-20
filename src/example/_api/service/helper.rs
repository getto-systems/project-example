use tonic::{metadata::MetadataValue, Request};

use crate::x_outside_feature::_common::metadata::METADATA_REQUEST_ID;

use crate::example::_api::service::data::ExampleServiceError;

pub fn infra_error(err: impl std::fmt::Display) -> ExampleServiceError {
    ExampleServiceError::InfraError(format!("service infra error; {}", err))
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
