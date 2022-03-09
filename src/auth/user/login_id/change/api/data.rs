use crate::{
    auth::user::login_id::kernel::data::ValidateLoginIdError,
    z_lib::repository::data::RepositoryError,
};

pub enum OverrideLoginIdError {
    InvalidLoginId(ValidateLoginIdError),
    UserNotFound,
    LoginIdAlreadyRegistered,
}

impl std::fmt::Display for OverrideLoginIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidLoginId(err) => write!(f, "invalid login id: {}", err),
            Self::UserNotFound => write!(f, "user not found"),
            Self::LoginIdAlreadyRegistered => write!(f, "new login id is already registered"),
        }
    }
}

pub enum OverrideLoginIdRepositoryError {
    RepositoryError(RepositoryError),
    UserNotFound,
    LoginIdAlreadyRegistered,
}
