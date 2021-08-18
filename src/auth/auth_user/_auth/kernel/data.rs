use crate::auth::auth_user::_common::kernel::data::{AuthUser, GrantedAuthRoles, RequireAuthRoles};

pub struct AuthPermission<'a> {
    granted_roles: &'a GrantedAuthRoles,
}

impl<'a> AuthPermission<'a> {
    pub fn new(user: &'a AuthUser) -> Self {
        Self {
            granted_roles: user.granted_roles(),
        }
    }

    pub fn has_enough_permission(&self, require_roles: &RequireAuthRoles) -> bool {
        match require_roles {
            RequireAuthRoles::Nothing => true,
            RequireAuthRoles::HasAny(roles) => {
                let granted_roles = self.granted_roles.as_roles();
                roles.iter().any(|role| granted_roles.contains(role))
            }
        }
    }
}
