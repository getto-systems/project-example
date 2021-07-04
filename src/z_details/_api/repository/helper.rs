use std::fmt::Display;

use super::data::RepositoryError;

pub fn infra_error(message: impl Display) -> RepositoryError {
    RepositoryError::InfraError(format!("{}", message))
}
