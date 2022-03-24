#[derive(Clone, PartialEq, Eq, Hash)]
pub enum AuthRole {
    User,
}

impl AuthRole {
    pub fn as_str(&self) -> &str {
        match self {
            Self::User => "user",
        }
    }
}

pub const AUTH_ROLE_ALL: [AuthRole; 1] = [AuthRole::User];
pub const AUTH_ROLE_USER: [AuthRole; 1] = [AuthRole::User];
