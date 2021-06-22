use std::fmt::{Display, Formatter};

pub enum AuthenticatePasswordResponse {
    UserNotFound(String),
    PasswordNotFound(String),
    PasswordNotMatched(String),
}

impl Display for AuthenticatePasswordResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::UserNotFound(_) => write!(f, "user not found"),
            Self::PasswordNotFound(_) => write!(f, "password not found"),
            Self::PasswordNotMatched(_) => write!(f, "password not matched"),
        }
    }
}
