use crate::auth::user::{
    kernel::data::{GrantedAuthRoles, ValidateGrantedAuthRolesError},
    login_id::kernel::data::ValidateLoginIdError, account::kernel::data::{AuthUserAttributes, ValidateAuthUserAttributesError},
};

#[derive(PartialEq, Eq)]
pub struct ModifyAuthUserAccountChanges {
    pub granted_roles: GrantedAuthRoles,
    pub attrs: AuthUserAttributes,
}

impl std::fmt::Display for ModifyAuthUserAccountChanges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "changes: {} / {}", self.granted_roles, self.attrs)
    }
}

pub enum ValidateModifyAuthUserAccountFieldsError {
    InvalidLoginId(ValidateLoginIdError),
    InvalidFrom(ValidateModifyAuthUserAccountChangesError),
    InvalidTo(ValidateModifyAuthUserAccountChangesError),
}

pub enum ValidateModifyAuthUserAccountChangesError {
    NotFound,
    InvalidGrantedRoles(ValidateGrantedAuthRolesError),
    InvalidAttrs(ValidateAuthUserAttributesError),
}

impl std::fmt::Display for ValidateModifyAuthUserAccountFieldsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidLoginId(err) => err.fmt(f),
            Self::InvalidFrom(err) => write!(f, "invalid from: {}", err),
            Self::InvalidTo(err) => write!(f, "invalid to: {}", err),
        }
    }
}

impl std::fmt::Display for ValidateModifyAuthUserAccountChangesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::NotFound => write!(f, "data not found"),
            Self::InvalidGrantedRoles(err) => err.fmt(f),
            Self::InvalidAttrs(err) => err.fmt(f),
        }
    }
}
