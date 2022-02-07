use sqlx::Error;

use crate::z_lib::repository::data::RepositoryError;

pub fn mysql_error(err: Error) -> RepositoryError {
    RepositoryError::InfraError(format!("{}", err))
}
