use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Duration, Utc};

use crate::auth::user::remote::kernel::data::{
    AuthPermission, AuthUser, AuthUserExtract, GrantedAuthRoles, RequireAuthRoles,
};

#[derive(Clone)]
pub struct AuthNonce(String);

impl AuthNonce {
    pub const fn restore(nonce: String) -> Self {
        Self(nonce)
    }

    pub fn extract(self) -> String {
        self.0
    }

    #[cfg(test)]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Clone)]
pub struct AuthToken(String);

impl AuthToken {
    pub const fn restore(token: String) -> Self {
        Self(token)
    }

    pub fn extract(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Clone)]
pub struct AuthTokenExtract {
    pub token: String,
    pub expires: i64,
}

pub struct AuthTokenEncoded {
    pub ticket_token: AuthTokenExtract,
    pub api_token: AuthTokenExtract,
    pub cloudfront_tokens: HashMap<CloudfrontTokenKind, AuthTokenExtract>,
}

#[derive(Eq, PartialEq, Hash)]
pub enum CloudfrontTokenKind {
    KeyPairId,
    Policy,
    Signature,
}

pub struct AuthTokenResponse {
    pub domain: String,
    pub message: AuthTokenMessage,
}

pub struct AuthTokenMessage {
    pub body: String,
    pub token: AuthTokenEncoded,
}

#[derive(Clone)]
pub struct AuthTicketExtract {
    pub ticket_id: String,
    pub user_id: String,
    pub granted_roles: HashSet<String>,
}

pub enum DecodeAuthTokenError {
    Expired,
    Invalid(String),
}

impl std::fmt::Display for DecodeAuthTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Expired => write!(f, "token expired"),
            Self::Invalid(err) => write!(f, "invalid token: {}", err),
        }
    }
}

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
        if AuthPermission::new(&self.user).has_enough_permission(&require_roles) {
            Ok(self)
        } else {
            Err(ValidateAuthRolesError::PermissionDenied(
                self.user.into_granted_roles(),
                require_roles,
            ))
        }
    }
}

impl std::fmt::Display for AuthTicket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} / {}", self.ticket_id, self.user)
    }
}

impl AuthTicketExtract {
    pub(in crate::auth) fn restore(self) -> AuthTicket {
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

impl std::fmt::Display for AuthTicketId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "ticket: {}", self.0)
    }
}

#[derive(Clone)]
pub struct AuthDateTime(DateTime<Utc>);

impl AuthDateTime {
    pub(in crate::auth) const fn restore(now: DateTime<Utc>) -> Self {
        Self(now)
    }

    pub fn extract(self) -> DateTime<Utc> {
        self.0
    }

    pub fn expires(self, duration: &ExpireDuration) -> ExpireDateTime {
        ExpireDateTime(self.0 + duration.0)
    }

    pub fn expansion_limit(self, duration: &ExpansionLimitDuration) -> ExpansionLimitDateTime {
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
    pub(in crate::auth) const fn restore(time: DateTime<Utc>) -> Self {
        Self(time)
    }

    pub fn has_elapsed(&self, now: &AuthDateTime) -> bool {
        self.0 < now.0
    }

    pub fn extract(self) -> DateTime<Utc> {
        self.0
    }
}

impl std::fmt::Display for ExpireDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.0.fmt(f)
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

impl ExpansionLimitDateTime {
    pub(in crate::auth) const fn restore(time: DateTime<Utc>) -> Self {
        Self(time)
    }

    pub fn extract(self) -> DateTime<Utc> {
        self.0
    }
}

impl std::fmt::Display for ExpansionLimitDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.0.fmt(f)
    }
}

#[derive(Clone, Copy)]
pub struct ExpansionLimitDuration(Duration);

impl ExpansionLimitDuration {
    pub fn with_duration(duration: Duration) -> Self {
        Self(duration)
    }
}

pub enum ValidateAuthRolesError {
    PermissionDenied(GrantedAuthRoles, RequireAuthRoles),
}

impl std::fmt::Display for ValidateAuthRolesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::PermissionDenied(granted_roles, require_roles) => {
                write!(
                    f,
                    "user permission denied; granted: {}, required: {}",
                    granted_roles, require_roles
                )
            }
        }
    }
}
