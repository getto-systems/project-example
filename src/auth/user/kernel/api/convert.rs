use crate::x_content::role::auth_role_set;

use crate::auth::user::kernel::data::{
    GrantedAuthRoles, GrantedAuthRolesExtract, ValidateGrantedAuthRolesError,
};

impl GrantedAuthRolesExtract for Vec<String> {
    fn validate(self) -> Result<GrantedAuthRoles, ValidateGrantedAuthRolesError> {
        let all_roles = auth_role_set();
        if self.iter().any(|role| !all_roles.contains(role)) {
            return Err(ValidateGrantedAuthRolesError::InvalidRole);
        }
        Ok(GrantedAuthRoles::restore(self.into_iter().collect()))
    }
}
