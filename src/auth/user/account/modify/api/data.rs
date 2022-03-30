use crate::auth::user::{
    kernel::data::{GrantedAuthRoles, ValidateGrantedAuthRolesError},
    login_id::kernel::data::ValidateLoginIdError,
};

#[derive(PartialEq, Eq)]
pub struct AuthUserAccountChanges {
    pub granted_roles: GrantedAuthRoles,
}

impl std::fmt::Display for AuthUserAccountChanges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "modify user: {}", self.granted_roles)
    }
}

pub enum ValidateAuthUserAccountError {
    InvalidLoginId(ValidateLoginIdError),
    InvalidFrom(ValidateAuthUserAccountChangesError),
    InvalidTo(ValidateAuthUserAccountChangesError),
}

pub enum ValidateAuthUserAccountChangesError {
    NotFound,
    InvalidGrantedRoles(ValidateGrantedAuthRolesError),
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

impl std::fmt::Display for ValidateAuthUserAccountChangesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::NotFound => write!(f, "data not found"),
            Self::InvalidGrantedRoles(err) => err.fmt(f),
        }
    }
}
