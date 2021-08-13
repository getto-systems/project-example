use std::{
    collections::HashSet,
    error::Error,
    fmt::{Display, Formatter},
};

use crate::{
    auth::auth_user::_common::kernel::data::{
        AuthUser, AuthUserExtract, GrantedAuthRoles, RequireAuthRoles,
    },
    z_details::{_auth::request::data::MetadataError, _common::repository::data::RepositoryError},
};

#[derive(Clone)]
pub struct AuthTicket {
    ticket_id: AuthTicketId,
    user: AuthUser,
}

impl AuthTicket {
    pub const fn new(ticket_id: AuthTicketId, user: AuthUser) -> Self {
        Self { ticket_id, user }
    }

    pub fn into_ticket_id(self) -> AuthTicketId {
        self.ticket_id
    }
    pub fn ticket_id_as_str(&self) -> &str {
        self.ticket_id.as_str()
    }

    pub fn into_user(self) -> AuthUser {
        self.user
    }

    pub fn extract(self) -> AuthTicketExtract {
        let user = self.user.extract();
        AuthTicketExtract {
            ticket_id: self.ticket_id.0,
            user_id: user.user_id,
            granted_roles: user.granted_roles,
        }
    }

    pub fn check_enough_permission(
        self,
        require_roles: RequireAuthRoles,
    ) -> Result<Self, ValidateAuthRolesError> {
        if self.user.has_enough_permission(&require_roles) {
            Ok(self)
        } else {
            Err(ValidateAuthRolesError::PermissionDenied(
                self.user.into_granted_roles(),
                require_roles,
            ))
        }
    }
}

impl Display for AuthTicket {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} / {}", self.ticket_id, self.user)
    }
}

pub struct AuthTicketExtract {
    pub ticket_id: String,
    pub user_id: String,
    pub granted_roles: HashSet<String>,
}

impl AuthTicketExtract {
    pub fn restore(self) -> AuthTicket {
        AuthTicket {
            ticket_id: AuthTicketId::new(self.ticket_id),
            user: AuthUserExtract {
                user_id: self.user_id,
                granted_roles: self.granted_roles,
            }
            .restore(),
        }
    }
}

#[derive(Clone)]
pub struct AuthTicketId(String);

impl AuthTicketId {
    pub const fn new(id: String) -> Self {
        Self(id)
    }

    pub fn extract(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Display for AuthTicketId {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "ticket: {}", self.0)
    }
}

#[derive(Debug)]
pub enum ValidateAuthNonceError {
    MetadataError(MetadataError),
    RepositoryError(RepositoryError),
    Conflict,
}

impl Display for ValidateAuthNonceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let label = "auth nonce error";
        match self {
            Self::MetadataError(err) => write!(f, "{}: {}", label, err),
            Self::RepositoryError(err) => write!(f, "{}: {}", label, err),
            Self::Conflict => write!(f, "{}: conflict", label),
        }
    }
}
impl Error for ValidateAuthNonceError {}

#[derive(Debug)]
pub enum ValidateAuthRolesError {
    PermissionDenied(GrantedAuthRoles, RequireAuthRoles),
}

impl Display for ValidateAuthRolesError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::PermissionDenied(granted_roles, require_roles) => {
                write!(
                    f,
                    "user permission denied: {}, {}",
                    granted_roles, require_roles
                )
            }
        }
    }
}
impl Error for ValidateAuthRolesError {}
