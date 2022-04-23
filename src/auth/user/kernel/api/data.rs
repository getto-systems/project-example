use std::collections::HashSet;

use crate::x_content::role::{AuthRole, AUTH_ROLE_ALL};

#[derive(Clone)]
pub struct AuthUser {
    user_id: AuthUserId,
    granted_roles: GrantedAuthRoles,
}

impl AuthUser {
    pub fn restore(user_id: AuthUserId, granted_roles: Option<GrantedAuthRoles>) -> Self {
        Self {
            user_id,
            granted_roles: granted_roles.unwrap_or(GrantedAuthRoles::empty()),
        }
    }

    pub fn into_user_id(self) -> AuthUserId {
        self.user_id
    }
    pub fn into_granted_roles(self) -> GrantedAuthRoles {
        self.granted_roles
    }

    #[cfg(test)]
    pub fn user_id(&self) -> &AuthUserId {
        &self.user_id
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

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct AuthUserId(String);

impl AuthUserId {
    pub(in crate::auth) const fn restore(user_id: String) -> Self {
        Self(user_id)
    }

    pub(in crate::auth) fn extract(self) -> String {
        self.0
    }
}

impl std::fmt::Display for AuthUserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "user: {}", self.0)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct GrantedAuthRoles(HashSet<AuthRole>);

impl GrantedAuthRoles {
    pub fn empty() -> Self {
        Self(HashSet::new())
    }

    pub fn validate(
        roles: impl GrantedAuthRolesExtract,
    ) -> Result<Self, ValidateGrantedAuthRolesError> {
        roles.validate()
    }

    pub(in crate::auth) fn restore(roles: HashSet<String>) -> Self {
        let mut granted_roles: HashSet<AuthRole> = HashSet::new();
        for ref role in AUTH_ROLE_ALL {
            if roles.contains(&role.as_str().to_owned()) {
                granted_roles.insert(role.clone());
            }
        }
        Self(granted_roles)
    }

    pub(in crate::auth) fn extract(self) -> HashSet<String> {
        self.0
            .into_iter()
            .map(|role| role.as_str().to_owned())
            .collect()
    }
    pub(in crate::auth) fn inner(&self) -> &HashSet<AuthRole> {
        &self.0
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

pub trait GrantedAuthRolesExtract {
    fn validate(self) -> Result<GrantedAuthRoles, ValidateGrantedAuthRolesError>;
}

pub enum ValidateGrantedAuthRolesError {
    InvalidRole,
}

impl std::fmt::Display for ValidateGrantedAuthRolesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidRole => write!(f, "invalid role"),
        }
    }
}

#[derive(Clone)]
pub enum RequireAuthRoles {
    Nothing,
    HasAny(HashSet<AuthRole>),
}

impl RequireAuthRoles {
    pub(in crate::auth) fn restore_has_any(roles: Vec<&str>) -> Self {
        let mut require_roles = HashSet::new();
        for ref role in AUTH_ROLE_ALL {
            if roles.contains(&role.as_str()) {
                require_roles.insert(role.clone());
            }
        }
        Self::HasAny(require_roles)
    }

    pub fn user() -> Self {
        Self::HasAny(vec![AuthRole::User].into_iter().collect())
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
