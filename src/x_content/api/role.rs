use crate::auth::data::RequireAuthRoles;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuthRole {
    AuthUser,
}

pub const AUTH_ROLE_ALL: [AuthRole; 1] = [AuthRole::AuthUser];

impl AuthRole {
    pub fn member(target: &str) -> Option<AuthRole> {
        for role in AUTH_ROLE_ALL {
            if target == role.as_str() {
                return Some(role);
            }
        }

        None
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::AuthUser => "auth-user",
        }
    }
}

impl RequireAuthRoles {
    pub fn user() -> Self {
        Self::HasAny(vec![AuthRole::AuthUser].into_iter().collect())
    }
}
