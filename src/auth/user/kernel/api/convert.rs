use crate::x_content::role::AuthRole;

use crate::auth::user::kernel::data::{
    GrantedAuthRoles, GrantedAuthRolesExtract, ValidateGrantedAuthRolesError,
};

impl GrantedAuthRolesExtract for Vec<String> {
    fn validate(self) -> Result<GrantedAuthRoles, ValidateGrantedAuthRolesError> {
        if self.iter().any(|role| AuthRole::member(role).is_none()) {
            return Err(ValidateGrantedAuthRolesError::InvalidRole);
        }
        Ok(GrantedAuthRoles::restore(self.into_iter().collect()))
    }
}
