use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
};

use crate::auth::auth_user::_common::kernel::data::{AuthRoles, AuthUser, GrantedAuthRoles};

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
