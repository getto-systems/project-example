use crate::auth::ticket::kernel::data::{AuthPermissionGranted, AuthTicketAttrs};

#[derive(Debug, Clone, PartialEq)]
pub struct AuthUser {
    pub user_id: AuthUserId,
    pub granted: AuthPermissionGranted,
}

impl std::fmt::Display for AuthUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} ({})", self.user_id, self.granted)
    }
}

impl Into<AuthTicketAttrs> for AuthUser {
    fn into(self) -> AuthTicketAttrs {
        AuthTicketAttrs {
            user_id: self.user_id,
            granted: self.granted,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
        write!(f, "user-id: {}", self.0)
    }
}
