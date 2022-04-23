use crate::auth::user::login_id::kernel::data::ValidateLoginIdError;

pub enum ValidateUnregisterAuthUserAccountFieldsError {
    InvalidLoginId(ValidateLoginIdError),
}

impl std::fmt::Display for ValidateUnregisterAuthUserAccountFieldsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidLoginId(err) => err.fmt(f),
        }
    }
}
