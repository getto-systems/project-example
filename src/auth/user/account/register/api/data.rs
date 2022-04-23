use crate::auth::user::{
    kernel::data::ValidateGrantedAuthRolesError, login_id::kernel::data::ValidateLoginIdError,
    password::reset::kernel::data::ValidateResetTokenDestinationError,
};

pub enum ValidateRegisterAuthUserAccountFieldsError {
    InvalidLoginId(ValidateLoginIdError),
    InvalidGrantedRoles(ValidateGrantedAuthRolesError),
    InvalidResetTokenDestination(ValidateResetTokenDestinationError),
}

impl std::fmt::Display for ValidateRegisterAuthUserAccountFieldsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidLoginId(err) => err.fmt(f),
            Self::InvalidGrantedRoles(err) => err.fmt(f),
            Self::InvalidResetTokenDestination(err) => err.fmt(f),
        }
    }
}
