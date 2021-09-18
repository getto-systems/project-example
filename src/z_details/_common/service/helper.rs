use crate::z_details::_common::service::data::ServiceAuthorizeError;

pub fn infra_error(err: impl std::fmt::Display) -> ServiceAuthorizeError {
    ServiceAuthorizeError::InfraError(format!("service infra error; {}", err))
}
