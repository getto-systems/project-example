use super::data::RepositoryError;

pub fn infra_error(message: impl std::fmt::Display) -> RepositoryError {
    RepositoryError::InfraError(format!("{}", message))
}
