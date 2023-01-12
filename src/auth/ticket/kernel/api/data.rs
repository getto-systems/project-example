use std::collections::{HashMap, HashSet};

use crate::x_content::{metadata::METADATA_AUTHORIZE_TOKEN, permission::AuthPermission};

use crate::{
    auth::{kernel::data::ExpireDateTime, user::kernel::data::AuthUserId},
    common::api::request::data::MetadataError,
    common::proxy::data::ProxyMetadataExtract,
};

#[derive(Clone)]
pub struct AuthTicket {
    pub ticket_id: AuthTicketId,
    pub attrs: AuthTicketAttrs,
}

#[derive(Clone)]
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

#[derive(Clone, PartialEq, Eq, Hash)]
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

#[derive(Clone, PartialEq, Eq)]
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

impl Default for AuthPermissionGranted {
    fn default() -> Self {
        Self(Default::default())
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

#[derive(Clone)]
pub enum AuthPermissionRequired {
    Nothing,
    HasSome(HashSet<AuthPermission>),
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

#[derive(Clone)]
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

#[derive(Clone)]
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

pub struct AuthToken {
    pub authenticate_token: (AuthenticateToken, ExpireDateTime),
    pub authorize_token: (AuthorizeToken, ExpireDateTime),
    pub cdn_token: (CdnToken, ExpireDateTime),
}

pub enum CdnToken {
    AWSCloudfront(AWSCloudfrontToken),
}

pub struct AWSCloudfrontToken {
    pub key_pair_id: String,
    pub policy: String,
    pub signature: String,
}
