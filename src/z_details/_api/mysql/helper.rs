use sqlx::Error;

use crate::z_details::_api::repository::data::RepositoryError;

pub fn mysql_error(err: Error) -> RepositoryError {
    RepositoryError::InfraError(format!("{}", err))
}
