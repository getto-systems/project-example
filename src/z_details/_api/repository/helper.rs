use std::fmt::Display;

use super::data::RepositoryError;

pub fn register_conflict_error(target: impl Display) -> RepositoryError {
    RepositoryError::InfraError(format!("register conflict; target: {}", target))
}
