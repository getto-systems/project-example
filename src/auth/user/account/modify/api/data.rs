use crate::{
    auth::user::login_id::kernel::data::ValidateLoginIdError,
    z_lib::repository::data::RepositoryError,
};

pub enum ModifyAuthUserAccountError {
    InvalidLoginId(ValidateLoginIdError),
    UserNotFound,
    Conflict,
    InvalidUser(ValidateAuthUserAccountError),
}

impl std::fmt::Display for ModifyAuthUserAccountError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidLoginId(err) => write!(f, "invalid login id: {}", err),
            Self::UserNotFound => write!(f, "user not found"),
            Self::LoginIdAlreadyRegistered => write!(f, "new login id is already registered"),
        }
    }
}

pub enum ValidateAuthUserAccountError {
    InvalidGrantedRole,
    InvalidEmail,
    EmptyEmail,
    TooLongEmail,
}

impl std::fmt::Display for ValidateAuthUserAccountError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidGrantedRole => write!(f, "invalid granted role"),
            Self::InvalidEmail => write!(f, "invalid email"),
            Self::EmptyEmail => write!(f, "empty email"),
            Self::TooLongEmail => write!(f, "too long email"),
        }
    }
}

pub enum ModifyAuthUserAccountRepositoryError {
    RepositoryError(RepositoryError),
    UserNotFound,
}
