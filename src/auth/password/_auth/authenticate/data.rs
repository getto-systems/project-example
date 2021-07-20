use std::fmt::{Display, Formatter};

use crate::auth::{
    login_id::_auth::data::ValidateLoginIdError,
    password::_auth::kernel::data::ValidatePasswordError,
};

pub enum AuthenticatePasswordError {
    InvalidLoginId(ValidateLoginIdError),
    InvalidPassword(ValidatePasswordError),
    PasswordNotFound,
    PasswordNotMatched,
}

impl Display for AuthenticatePasswordError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidLoginId(err) => write!(f, "invalid login id; {}", err),
            Self::InvalidPassword(err) => write!(f, "invalid password; {}", err),
            Self::PasswordNotFound => write!(f, "password not found"),
            Self::PasswordNotMatched => write!(f, "password not matched"),
        }
    }
}
