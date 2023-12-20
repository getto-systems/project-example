use std::collections::{HashMap, HashSet};

use crate::x_content::{metadata::METADATA_AUTHORIZE_TOKEN, permission::AuthPermission};

use crate::{
    auth::{
        kernel::data::ExpireDateTime,
        user::kernel::data::{AuthUser, AuthUserId},
    },
    common::{api::request::data::MetadataError, proxy::data::ProxyMetadataExtract},
};

#[derive(Debug, PartialEq)]
pub struct AuthenticateSuccess(AuthUser);

impl AuthenticateSuccess {
    pub fn new(user: AuthUser) -> Self {
        Self(user)
    }
}

impl std::fmt::Display for AuthenticateSuccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "authenticate success; {}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AuthTicket {
    pub ticket_id: AuthTicketId,
    pub attrs: AuthTicketAttrs,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AuthTicketAttrs {
    pub user_id: AuthUserId,
    pub granted: AuthPermissionGranted,
}

impl AuthTicket {
    #[cfg(test)]
    pub fn standard() -> Self {
        Self {
            ticket_id: AuthTicketId::restore("ticket-id".to_owned()),
            attrs: AuthTicketAttrs {
                user_id: AuthUserId::restore("user-id".to_owned()),
                granted: AuthPermissionGranted::default(),
            },
        }
    }
}

impl std::fmt::Display for AuthTicket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} / {}", self.ticket_id, self.attrs)
    }
}

impl std::fmt::Display for AuthTicketAttrs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} ({})", self.user_id, self.granted)
    }
}

impl Into<AuthTicketAttrs> for AuthenticateSuccess {
    fn into(self) -> AuthTicketAttrs {
        AuthTicketAttrs {
            user_id: self.0.user_id,
            granted: self.0.granted,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AuthTicketId(String);

impl AuthTicketId {
    pub const fn restore(id: String) -> Self {
        Self(id)
    }

    pub fn extract(self) -> String {
        self.0
    }
}

impl std::fmt::Display for AuthTicketId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "ticket: {}", self.0)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct AuthPermissionGranted(HashSet<AuthPermission>);

impl AuthPermissionGranted {
    pub fn convert(
        value: impl AuthPermissionGrantedExtract,
    ) -> Result<Self, ValidateAuthPermissionGrantedError> {
        value.convert()
    }

    pub(in crate::auth) fn restore(value: HashSet<AuthPermission>) -> Self {
        Self(value)
    }

    pub(in crate::auth) fn extract(self) -> HashSet<String> {
        self.0
            .into_iter()
            .map(|permission| permission.extract())
            .collect()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn all_contains(&self, permissions: &Vec<AuthPermission>) -> bool {
        permissions
            .iter()
            .all(|permission| self.0.contains(permission))
    }

    pub fn has_enough_permission(
        &self,
        required: &AuthPermissionRequired,
    ) -> Result<(), AuthPermissionError> {
        if match required {
            AuthPermissionRequired::Nothing => true,
            AuthPermissionRequired::HasSome(permissions) => permissions
                .iter()
                .any(|permission| self.0.contains(permission)),
        } {
            Ok(())
        } else {
            Err(AuthPermissionError::PermissionDenied(
                self.clone(),
                required.clone(),
            ))
        }
    }
}

impl std::fmt::Display for AuthPermissionGranted {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "granted: [{}]",
            self.0
                .iter()
                .map(|permission| permission.extract())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

pub trait AuthPermissionGrantedExtract {
    fn convert(self) -> Result<AuthPermissionGranted, ValidateAuthPermissionGrantedError>;
}

#[derive(Debug)]
pub enum ValidateAuthPermissionGrantedError {
    InvalidPermission,
}

impl std::fmt::Display for ValidateAuthPermissionGrantedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::InvalidPermission => write!(f, "auth-permission-granted: invalid"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AuthPermissionRequired {
    Nothing,
    HasSome(HashSet<AuthPermission>),
}

impl AuthPermissionRequired {
    pub fn union(self, other: Self) -> Self {
        match (self, other) {
            (Self::Nothing, Self::Nothing) => Self::Nothing,
            (Self::Nothing, Self::HasSome(permissions)) => Self::HasSome(permissions),
            (Self::HasSome(permissions), Self::Nothing) => Self::HasSome(permissions),
            (Self::HasSome(self_permissions), Self::HasSome(other_permissions)) => Self::HasSome(
                self_permissions
                    .union(&other_permissions)
                    .map(Clone::clone)
                    .collect(),
            ),
        }
    }
}

impl std::fmt::Display for AuthPermissionRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Nothing => write!(f, "require: nothing"),
            Self::HasSome(permissions) => write!(
                f,
                "require: some [{}]",
                permissions
                    .iter()
                    .map(|permission| permission.extract())
                    .collect::<Vec<String>>()
                    .join(",")
            ),
        }
    }
}

pub trait AuthPermissionRequiredExtract {
    fn convert(self) -> Result<AuthPermissionRequired, ValidateAuthPermissionError>;
}

#[derive(Debug)]
pub enum AuthPermissionError {
    PermissionDenied(AuthPermissionGranted, AuthPermissionRequired),
}

impl std::fmt::Display for AuthPermissionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::PermissionDenied(granted, required) => {
                write!(f, "permission denied; {}, {}", granted, required)
            }
        }
    }
}

#[derive(Debug)]
pub enum ValidateAuthPermissionError {
    Invalid,
}

impl std::fmt::Display for ValidateAuthPermissionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Invalid => write!(f, "invalid permission"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AuthenticateToken(String);

impl AuthenticateToken {
    pub const fn restore(token: String) -> Self {
        Self(token)
    }

    pub fn extract(self) -> String {
        self.0
    }
}

pub trait AuthenticateTokenExtract {
    fn convert(self) -> Result<AuthenticateToken, ValidateAuthenticateTokenError>;
}

impl AuthenticateTokenExtract for AuthenticateToken {
    fn convert(self) -> Result<AuthenticateToken, ValidateAuthenticateTokenError> {
        Ok(self)
    }
}

#[derive(Debug)]
pub enum ValidateAuthenticateTokenError {
    NotFound,
    MetadataError(MetadataError),
}

impl std::fmt::Display for ValidateAuthenticateTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::NotFound => write!(f, "data not found"),
            Self::MetadataError(err) => err.fmt(f),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DecodeAuthenticateTokenError {
    Expired,
    Invalid(String),
}

impl std::fmt::Display for DecodeAuthenticateTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Expired => write!(f, "token expired"),
            Self::Invalid(err) => write!(f, "invalid token: {}", err),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AuthorizeToken(String);

impl AuthorizeToken {
    pub const fn restore(token: String) -> Self {
        Self(token)
    }

    pub fn extract(self) -> String {
        self.0
    }
}

pub trait AuthorizeTokenExtract {
    fn convert(self) -> Result<AuthorizeToken, ValidateAuthorizeTokenError>;
}

impl AuthorizeTokenExtract for AuthorizeToken {
    fn convert(self) -> Result<AuthorizeToken, ValidateAuthorizeTokenError> {
        Ok(self)
    }
}

impl<T: AuthorizeTokenExtract + Send> ProxyMetadataExtract for T {
    fn convert(self) -> Result<HashMap<&'static str, String>, MetadataError> {
        match self.convert() {
            Ok(token) => Ok(vec![(METADATA_AUTHORIZE_TOKEN, token.extract())]
                .into_iter()
                .collect()),
            Err(err) => match err {
                ValidateAuthorizeTokenError::NotFound => Ok(Default::default()),
                ValidateAuthorizeTokenError::MetadataError(err) => Err(err),
            },
        }
    }
}

#[derive(Debug)]
pub enum ValidateAuthorizeTokenError {
    NotFound,
    MetadataError(MetadataError),
}

impl std::fmt::Display for ValidateAuthorizeTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::NotFound => write!(f, "data not found"),
            Self::MetadataError(err) => err.fmt(f),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DecodeAuthorizeTokenError {
    Expired,
    Invalid(String),
}

impl std::fmt::Display for DecodeAuthorizeTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Expired => write!(f, "token expired"),
            Self::Invalid(err) => write!(f, "invalid token: {}", err),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AuthToken {
    pub authenticate_token: (AuthenticateToken, ExpireDateTime),
    pub authorize_token: (AuthorizeToken, ExpireDateTime),
    pub cdn_token: (CdnToken, ExpireDateTime),
}

#[derive(Debug, PartialEq)]
pub enum CdnToken {
    AWSCloudfront(AWSCloudfrontToken),
}

#[derive(Debug, PartialEq)]
pub struct AWSCloudfrontToken {
    pub key_pair_id: String,
    pub policy: String,
    pub signature: String,
}
