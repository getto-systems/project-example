use mysql::Error;

use crate::z_details::_api::repository::data::RepositoryError;

pub fn infra_error(err: Error) -> RepositoryError {
    RepositoryError::InfraError(format!("{}", err))
}
