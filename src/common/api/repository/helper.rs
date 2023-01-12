use super::data::RepositoryError;

pub fn repository_infra_error(
    label: &'static str,
    message: impl std::fmt::Display,
) -> RepositoryError {
    RepositoryError::InfraError(format!("{}; {}", label, message))
}
