use crate::auth::user::{
    login_id::kernel::data::ValidateLoginIdError, password::kernel::data::ValidatePasswordError,
};

pub enum ValidateAuthenticateWithPasswordFieldsError {
    InvalidLoginId(ValidateLoginIdError),
    InvalidPassword(ValidatePasswordError),
}

impl std::fmt::Display for ValidateAuthenticateWithPasswordFieldsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidLoginId(err) => err.fmt(f),
            Self::InvalidPassword(err) => write!(f, "password: {}", err),
        }
    }
}
