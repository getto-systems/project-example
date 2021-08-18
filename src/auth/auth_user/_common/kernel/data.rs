use std::{
    collections::{hash_set::Iter, HashSet},
    fmt::{Display, Formatter},
};

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

impl Display for AuthUser {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
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

    fn extract(self) -> String {
        self.0
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
    fn restore(granted_roles: impl GrantedAuthRolesExtract) -> Self {
        granted_roles.restore()
    }

    fn extract(self) -> HashSet<String> {
        self.0.extract()
    }

    pub fn as_roles(&self) -> &AuthRoles {
        &self.0
    }
}

trait GrantedAuthRolesExtract {
    fn restore(self) -> GrantedAuthRoles;
}

impl GrantedAuthRolesExtract for HashSet<String> {
    fn restore(self) -> GrantedAuthRoles {
        GrantedAuthRoles(AuthRoles::restore(self))
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
        Self::HasAny(AuthRoles::restore(hash_set))
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
    pub fn restore(roles: impl AuthRolesExtract) -> Self {
        roles.restore()
    }

    pub fn extract(self) -> HashSet<String> {
        self.0
    }

    pub fn iter(&self) -> Iter<'_, String> {
        self.0.iter()
    }
    pub fn contains(&self, role: &str) -> bool {
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

pub trait AuthRolesExtract {
    fn restore(self) -> AuthRoles;
}

impl AuthRolesExtract for HashSet<String> {
    fn restore(self) -> AuthRoles {
        AuthRoles(self)
    }
}
