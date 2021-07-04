use std::fmt::Display;

use super::data::RepositoryError;

pub fn infra_error(message: impl Display) -> RepositoryError {
    RepositoryError::InfraError(format!("{}", message))
}

// TODO 多分 test 用
pub fn register_conflict_error(target: impl Display) -> RepositoryError {
    RepositoryError::InfraError(format!("register conflict; target: {}", target))
}
