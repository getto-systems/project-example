use std::collections::HashSet;

#[derive(Clone)]
pub struct AuthUser {
    user_id: AuthUserId,
    granted_roles: GrantedAuthRoles,
}

impl AuthUser {
    pub fn into_user_id(self) -> AuthUserId {
        self.user_id
    }
    pub fn into_granted_roles(self) -> GrantedAuthRoles {
        self.granted_roles
    }

    pub fn granted_roles(&self) -> &GrantedAuthRoles {
        &self.granted_roles
    }

    pub fn extract(self) -> AuthUserExtract {
        AuthUserExtract {
            user_id: self.user_id.extract(),
            granted_roles: self.granted_roles.extract(),
        }
    }
}

impl std::fmt::Display for AuthUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} ({})", self.user_id, self.granted_roles)
    }
}

pub struct AuthUserExtract {
    pub user_id: String,
    pub granted_roles: HashSet<String>,
}

impl AuthUserExtract {
    pub(in crate::auth) fn restore(self) -> AuthUser {
        AuthUser {
            user_id: AuthUserId::restore(self.user_id),
            granted_roles: GrantedAuthRoles::restore(self.granted_roles),
        }
    }
}

#[derive(Clone)]
pub struct AuthUserId(String);

impl AuthUserId {
    pub(in crate::auth) const fn restore(user_id: String) -> Self {
        Self(user_id)
    }

    pub(in crate::auth) fn extract(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl std::fmt::Display for AuthUserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "user: {}", self.0)
    }
}

#[derive(Clone)]
pub struct GrantedAuthRoles(HashSet<String>);

impl GrantedAuthRoles {
    pub(in crate::auth) fn restore(granted_roles: HashSet<String>) -> Self {
        Self(granted_roles)
    }

    pub(in crate::auth) fn extract(self) -> HashSet<String> {
        self.0
    }

    pub fn has_enough_permission(&self, require_roles: &RequireAuthRoles) -> bool {
        match require_roles {
            RequireAuthRoles::Nothing => true,
            RequireAuthRoles::HasAny(roles) => roles.iter().any(|role| self.0.contains(role)),
        }
    }
}

impl std::fmt::Display for GrantedAuthRoles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "granted: [{}]",
            self.0
                .iter()
                .map(|role| role.as_str())
                .collect::<Vec<&str>>()
                .join(",")
        )
    }
}

pub struct GrantedAuthRolesBasket(HashSet<String>);

impl GrantedAuthRolesBasket {
    pub fn new(granted_roles: HashSet<String>) -> Self {
        Self(granted_roles)
    }

    pub fn extract(self) -> HashSet<String> {
        self.0
    }
}

#[derive(Clone)]
pub enum RequireAuthRoles {
    Nothing,
    HasAny(HashSet<String>),
}

impl RequireAuthRoles {
    // TODO 例えばこんな感じで許可する role を構築するヘルパーを追加していく
    // TODO ここが role を列挙する場所になるけど、これは適切な場所ではない気がする
    // TODO 特に、user の role 管理でこの値が必要になるはずで・・・
    pub fn manage_auth_user() -> Self {
        Self::has_any(&["manage_auth_user"])
    }

    pub fn has_any(roles: &[&str]) -> Self {
        let mut set = HashSet::new();
        roles.into_iter().for_each(|role| {
            set.insert(role.to_string());
        });
        Self::HasAny(set)
    }
}

impl std::fmt::Display for RequireAuthRoles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            RequireAuthRoles::Nothing => write!(f, "require: nothing"),
            RequireAuthRoles::HasAny(roles) => write!(
                f,
                "require: any [{}]",
                roles
                    .iter()
                    .map(|role| role.as_str())
                    .collect::<Vec<&str>>()
                    .join(",")
            ),
        }
    }
}