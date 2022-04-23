use crate::auth::user::login_id::kernel::data::ValidateLoginIdError;

pub enum ValidateOverrideLoginIdFieldsError {
    InvalidCurrentLoginId(ValidateLoginIdError),
    InvalidNewLoginId(ValidateLoginIdError),
}

impl std::fmt::Display for ValidateOverrideLoginIdFieldsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCurrentLoginId(err) => write!(f, "current: {}", err),
            Self::InvalidNewLoginId(err) => write!(f, "new: {}", err),
        }
    }
}