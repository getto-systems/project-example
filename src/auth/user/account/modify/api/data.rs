use crate::auth::user::{
    account::kernel::data::ValidateAuthUserAccountError,
    login_id::kernel::data::ValidateLoginIdError,
};

pub enum ValidateModifyAuthUserAccountFieldsError {
    InvalidLoginId(ValidateLoginIdError),
    InvalidFrom(ValidateAuthUserAccountError),
    InvalidTo(ValidateAuthUserAccountError),
}

impl std::fmt::Display for ValidateModifyAuthUserAccountFieldsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidLoginId(err) => err.fmt(f),
            Self::InvalidFrom(err) => write!(f, "invalid from; {}", err),
            Self::InvalidTo(err) => write!(f, "invalid to; {}", err),
        }
    }
}
