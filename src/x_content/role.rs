#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuthRole {
    User,
}

pub const AUTH_ROLE_ALL: [AuthRole; 1] = [AuthRole::User];

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
            Self::User => "user",
        }
    }
}
