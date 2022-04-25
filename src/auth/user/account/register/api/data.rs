use crate::auth::user::{
    account::kernel::data::ValidateAuthUserAttributesError,
    kernel::data::ValidateGrantedAuthRolesError, login_id::kernel::data::ValidateLoginIdError,
    password::reset::kernel::data::ValidateResetTokenDestinationError,
};

pub enum ValidateRegisterAuthUserAccountFieldsError {
    InvalidLoginId(ValidateLoginIdError),
    InvalidGrantedRoles(ValidateGrantedAuthRolesError),
    InvalidResetTokenDestination(ValidateResetTokenDestinationError),
    InvalidAttrs(ValidateAuthUserAttributesError),
}

impl std::fmt::Display for ValidateRegisterAuthUserAccountFieldsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidLoginId(err) => write!(f, "login-id: {}", err),
            Self::InvalidGrantedRoles(err) => write!(f, "granted-roles: {}", err),
            Self::InvalidResetTokenDestination(err) => {
                write!(f, "reset-token destination: {}", err)
            }
            Self::InvalidAttrs(err) => write!(f, "attrs: {}", err),
        }
    }
}
