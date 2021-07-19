use std::fmt::Display;

use crate::auth::_api::service::data::ServiceError;

pub fn infra_error(err: impl Display) -> ServiceError {
    ServiceError::InfraError(format!("service infra error; {}", err))
}
