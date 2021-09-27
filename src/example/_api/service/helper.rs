use crate::example::_api::service::data::ExampleServiceError;

pub fn infra_error(err: impl std::fmt::Display) -> ExampleServiceError {
    ExampleServiceError::InfraError(format!("service infra error; {}", err))
}
