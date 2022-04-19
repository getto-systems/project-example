use std::collections::HashSet;

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

pub fn auth_role_set() -> HashSet<String> {
    let mut set = HashSet::new();
    AUTH_ROLE_ALL.iter().for_each(|role| {
        set.insert(role.as_str().to_owned());
    });
    set
}
