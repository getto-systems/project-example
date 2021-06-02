use std::{
    collections::HashSet,
    error::Error,
    fmt::{Display, Formatter},
};

use chrono::{DateTime, Duration, Utc};

use crate::auth::auth_user::_api::kernel::data::{
    AuthUser, AuthUserExtract, GrantedAuthRoles, RequireAuthRoles,
};
use crate::z_details::_api::{repository::data::RepositoryError, request::data::HeaderError};

#[derive(Clone)]
pub struct AuthNonceValue(String);

impl AuthNonceValue {
    pub const fn new(nonce: String) -> Self {
        Self(nonce)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Clone)]
pub struct AuthToken {
    value: AuthTokenValue,
    expires: ExpireDateTime,
}

#[derive(Clone)]
pub struct AuthTokenExtract {
    pub value: String,
    pub expires: ExpireDateTime,
}

impl AuthToken {
    pub fn new(token: AuthTokenExtract) -> Self {
        Self {
            value: AuthTokenValue(token.value),
            expires: token.expires,
        }
    }

    pub fn extract(self) -> AuthTokenExtract {
        AuthTokenExtract {
            value: self.value.0,
            expires: self.expires,
        }
    }

    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }
}

#[derive(Clone)]
pub struct AuthTokenValue(String);

impl AuthTokenValue {
    pub const fn new(token: String) -> Self {
        Self(token)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Clone)]
pub struct AuthTicket {
    id: AuthTicketId,
    user: AuthUser,
}

impl AuthTicket {
    pub const fn new(id: AuthTicketId, user: AuthUser) -> Self {
        Self { id, user }
    }
    pub fn from_extract(ticket: AuthTicketExtract) -> Self {
        Self {
            id: AuthTicketId::new(ticket.auth_ticket_id),
            user: AuthUser::from_extract(AuthUserExtract {
                id: ticket.user_id,
                granted_roles: ticket.granted_roles,
            }),
        }
    }

    pub fn into_id(self) -> AuthTicketId {
        self.id
    }
    pub fn id_as_str(&self) -> &str {
        self.id.as_str()
    }

    pub fn into_granted_roles(self) -> GrantedAuthRoles {
        self.user.into_granted_roles()
    }

    pub fn extract(self) -> AuthTicketExtract {
        let user = self.user.extract();
        AuthTicketExtract {
            auth_ticket_id: self.id.0,
            user_id: user.id,
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
        write!(f, "auth-ticket: {} / {}", self.id, self.user)
    }
}

pub struct AuthTicketExtract {
    pub auth_ticket_id: String,
    pub user_id: String,
    pub granted_roles: HashSet<String>,
}

#[derive(Clone)]
pub struct AuthTicketId(String);

impl AuthTicketId {
    pub const fn new(id: String) -> Self {
        Self(id)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Display for AuthTicketId {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "auth-ticket: {}", self.0)
    }
}

pub struct AuthDateTime(DateTime<Utc>);

impl AuthDateTime {
    pub const fn from_now(now: DateTime<Utc>) -> Self {
        Self(now)
    }

    pub fn expires(self, duration: &ExpireDuration) -> ExpireDateTime {
        ExpireDateTime(self.0 + duration.0)
    }

    pub fn limit(self, duration: &ExpansionLimitDuration) -> ExpansionLimitDateTime {
        ExpansionLimitDateTime(self.0 + duration.0)
    }

    pub fn expires_with_limit(
        self,
        duration: &ExpireDuration,
        limit: ExpansionLimitDateTime,
    ) -> ExpireDateTime {
        let expires = self.0 + duration.0;
        if expires > limit.0 {
            ExpireDateTime(limit.0)
        } else {
            ExpireDateTime(expires)
        }
    }
}

#[derive(Clone)]
pub struct ExpireDateTime(DateTime<Utc>);

impl ExpireDateTime {
    pub fn has_elapsed(&self, now: AuthDateTime) -> bool {
        self.0 < now.0
    }

    pub fn timestamp(&self) -> i64 {
        self.0.timestamp()
    }
}

#[derive(Clone, Copy)]
pub struct ExpireDuration(Duration);

impl ExpireDuration {
    pub fn with_duration(duration: Duration) -> Self {
        Self(duration)
    }
}

#[derive(Clone)]
pub struct ExpansionLimitDateTime(DateTime<Utc>);

#[derive(Clone, Copy)]
pub struct ExpansionLimitDuration(Duration);

impl ExpansionLimitDuration {
    pub fn with_duration(duration: Duration) -> Self {
        Self(duration)
    }
}

#[derive(Debug)]
pub enum ValidateAuthNonceError {
    HeaderError(HeaderError),
    RepositoryError(RepositoryError),
    Conflict,
}

impl Display for ValidateAuthNonceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let label = "auth nonce error";
        match self {
            Self::HeaderError(err) => write!(f, "{}: {}", label, err),
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
