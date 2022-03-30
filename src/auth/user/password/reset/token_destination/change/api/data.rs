use crate::auth::user::{
    kernel::data::{GrantedAuthRoles, ValidateGrantedAuthRolesError},
    login_id::kernel::data::ValidateLoginIdError,
    password::reset::kernel::data::{ResetTokenDestination, ValidateResetTokenDestinationError},
};

#[derive(PartialEq, Eq)]
pub struct ModifyAuthUserAccountData {
    pub granted_roles: GrantedAuthRoles,
    pub reset_token_destination: ResetTokenDestination,
}

impl std::fmt::Display for ModifyAuthUserAccountData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "modify user: {} / {}",
            self.granted_roles, self.reset_token_destination
        )
    }
}

pub enum ValidateAuthUserAccountError {
    InvalidLoginId(ValidateLoginIdError),
    InvalidFrom(ValidateAuthUserAccountDataError),
    InvalidTo(ValidateAuthUserAccountDataError),
}

pub enum ValidateAuthUserAccountDataError {
    NotFound,
    InvalidGrantedRoles(ValidateGrantedAuthRolesError),
    InvalidResetTokenDestination(ValidateResetTokenDestinationError),
}

impl std::fmt::Display for ValidateAuthUserAccountError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidLoginId(err) => err.fmt(f),
            Self::InvalidFrom(err) => write!(f, "invalid from: {}", err),
            Self::InvalidTo(err) => write!(f, "invalid to: {}", err),
        }
    }
}

impl std::fmt::Display for ValidateAuthUserAccountDataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::NotFound => write!(f, "data not found"),
            Self::InvalidGrantedRoles(err) => err.fmt(f),
            Self::InvalidResetTokenDestination(err) => err.fmt(f),
        }
    }
}
