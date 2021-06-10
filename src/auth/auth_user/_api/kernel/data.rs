use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
};

#[derive(Clone)]
pub struct AuthUser {
    id: AuthUserId,
    granted_roles: GrantedAuthRoles,
}

impl AuthUser {
    pub fn from_extract(user: AuthUserExtract) -> Self {
        Self {
            id: AuthUserId::new(user.id),
            granted_roles: GrantedAuthRoles::from_extract(user.granted_roles),
        }
    }

    pub fn id_as_str(&self) -> &str {
        self.id.0.as_str()
    }

    pub fn into_granted_roles(self) -> GrantedAuthRoles {
        self.granted_roles
    }

    pub fn extract(self) -> AuthUserExtract {
        AuthUserExtract {
            id: self.id.0,
            granted_roles: self.granted_roles.0 .0,
        }
    }

    pub fn has_enough_permission(&self, require_roles: &RequireAuthRoles) -> bool {
        self.granted_roles.has_enough_permission(require_roles)
    }
}

impl Display for AuthUser {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} ({})", self.id, self.granted_roles)
    }
}

pub struct AuthUserExtract {
    pub id: String,
    pub granted_roles: HashSet<String>,
}

#[derive(Clone)]
pub struct AuthUserId(String);

impl AuthUserId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Display for AuthUserId {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "user: {}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct GrantedAuthRoles(AuthRoles);

impl GrantedAuthRoles {
    fn from_extract(roles: HashSet<String>) -> Self {
        Self(AuthRoles(roles))
    }

    pub fn has_enough_permission(&self, require_roles: &RequireAuthRoles) -> bool {
        match require_roles {
            RequireAuthRoles::Nothing => true,
            RequireAuthRoles::HasAny(roles) => roles.any(|role| self.0.contains(role)),
        }
    }

    pub fn extract(self) -> HashSet<String> {
        self.0 .0
    }
}

impl Display for GrantedAuthRoles {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "granted: {}", self.0)
    }
}

#[derive(Debug, Clone)]
pub enum RequireAuthRoles {
    Nothing,
    HasAny(AuthRoles),
}

impl RequireAuthRoles {
    // TODO 例えばこんな感じで許可する role を構築するヘルパーを追加していく
    // TODO ここが role を列挙する場所になるけど、これは適切な場所ではない気がする
    // TODO 特に、user の role 管理でこの値が必要になるはずで・・・
    pub fn user() -> Self {
        Self::api(&["user"])
    }

    // admin ロールを持っていれば api アクセスが可能
    fn api(roles: &[&str]) -> Self {
        let mut roles = Vec::from(roles);
        roles.push("admin");
        Self::has_any(roles.as_ref())
    }

    pub fn has_any(roles: &[&str]) -> Self {
        let mut hash_set = HashSet::new();
        roles.iter().for_each(|role| {
            hash_set.insert(role.to_string());
        });
        Self::HasAny(AuthRoles(hash_set))
    }
}

impl Display for RequireAuthRoles {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            RequireAuthRoles::Nothing => write!(f, "require: nothing"),
            RequireAuthRoles::HasAny(roles) => write!(f, "require: any {}", roles),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AuthRoles(HashSet<String>);

impl AuthRoles {
    fn any(&self, f: impl FnMut(&String) -> bool) -> bool {
        self.0.iter().any(f)
    }
    fn contains(&self, role: &String) -> bool {
        self.0.contains(role)
    }
}

impl Display for AuthRoles {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|role| role.as_str())
                .collect::<Vec<&str>>()
                .join(",")
        )
    }
}
