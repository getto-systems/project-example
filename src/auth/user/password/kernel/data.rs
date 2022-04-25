use crate::z_lib::validate::data::ValidateTextError;

pub enum ValidatePasswordError {
    Text(ValidateTextError),
}

impl std::fmt::Display for ValidatePasswordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Text(err) => err.fmt(f),
        }
    }
}

pub enum PasswordHashError {
    InfraError(String),
}

impl std::fmt::Display for PasswordHashError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InfraError(err) => write!(f, "password hash error: {}", err),
        }
    }
}
