use sqlx::Error;

use crate::z_lib::remote::repository::data::RepositoryError;

pub fn mysql_error(err: Error) -> RepositoryError {
    RepositoryError::InfraError(format!("{}", err))
}
