use mysql::{AccessMode, Error, TxOpts};

use crate::z_details::_api::repository::data::RepositoryError;

pub fn mysql_error(err: Error) -> RepositoryError {
    RepositoryError::InfraError(format!("{}", err))
}

pub fn read_only_transaction() -> TxOpts {
    TxOpts::default().set_access_mode(Some(AccessMode::ReadOnly))
}
