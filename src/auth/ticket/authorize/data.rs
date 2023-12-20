use crate::{
    auth::ticket::kernel::data::{
        AuthPermissionError, AuthPermissionGranted, AuthTicketAttrs, DecodeAuthorizeTokenError,
        ValidateAuthPermissionError, ValidateAuthorizeTokenError,
    },
    common::api::repository::data::RepositoryError,
};

#[derive(Debug, PartialEq)]
pub struct CheckAuthorizeTokenSuccess(AuthPermissionGranted);

impl CheckAuthorizeTokenSuccess {
    pub(in crate::auth) fn new(granted: AuthPermissionGranted) -> Self {
        Self(granted)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AuthorizeSuccess(AuthTicketAttrs);

impl AuthorizeSuccess {
    pub(in crate::auth) fn new(attrs: AuthTicketAttrs) -> Self {
        Self(attrs)
    }

    pub(in crate::auth) fn extract(self) -> AuthTicketAttrs {
        self.0
    }
}

impl std::fmt::Display for AuthorizeSuccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "authorized; {}", self.0)
    }
}

#[derive(Debug)]
pub enum CheckAuthorizeTokenError {
    Invalid(ValidateAuthorizeTokenError),
    DecodeError(DecodeAuthorizeTokenError),
    PermissionError(AuthPermissionError),
}

mod check_authorize_token_error {
    const ERROR: &'static str = "check authorize-token error";

    impl std::fmt::Display for super::CheckAuthorizeTokenError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Invalid(err) => write!(f, "{}; {}", ERROR, err),
                Self::DecodeError(err) => write!(f, "{}; {}", ERROR, err),
                Self::PermissionError(err) => write!(f, "{}; {}", ERROR, err),
            }
        }
    }
}

impl From<ValidateAuthorizeTokenError> for CheckAuthorizeTokenError {
    fn from(value: ValidateAuthorizeTokenError) -> Self {
        Self::Invalid(value)
    }
}

impl From<DecodeAuthorizeTokenError> for CheckAuthorizeTokenError {
    fn from(value: DecodeAuthorizeTokenError) -> Self {
        Self::DecodeError(value)
    }
}

impl From<AuthPermissionError> for CheckAuthorizeTokenError {
    fn from(value: AuthPermissionError) -> Self {
        Self::PermissionError(value)
    }
}

#[derive(Debug)]
pub enum AuthorizeError {
    TicketNotFound,
    TicketHasExpired,
    UserNotFound,
    Invalid(ValidateAuthorizeFieldsError),
    DecodeError(DecodeAuthorizeTokenError),
    PermissionError(AuthPermissionError),
    RepositoryError(RepositoryError),
}

mod authorize_error {
    const ERROR: &'static str = "authorize error";

    impl std::fmt::Display for super::AuthorizeError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::TicketNotFound => write!(f, "{}; ticket not found", ERROR),
                Self::TicketHasExpired => write!(f, "{}; ticket has expired", ERROR),
                Self::UserNotFound => write!(f, "{}; user not found", ERROR),
                Self::Invalid(err) => write!(f, "{}; {}", ERROR, err),
                Self::DecodeError(err) => write!(f, "{}; {}", ERROR, err),
                Self::PermissionError(err) => write!(f, "{}; {}", ERROR, err),
                Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
            }
        }
    }
}

impl From<ValidateAuthorizeFieldsError> for AuthorizeError {
    fn from(value: ValidateAuthorizeFieldsError) -> Self {
        Self::Invalid(value)
    }
}

impl From<DecodeAuthorizeTokenError> for AuthorizeError {
    fn from(value: DecodeAuthorizeTokenError) -> Self {
        Self::DecodeError(value)
    }
}

impl From<AuthPermissionError> for AuthorizeError {
    fn from(value: AuthPermissionError) -> Self {
        Self::PermissionError(value)
    }
}

impl From<RepositoryError> for AuthorizeError {
    fn from(value: RepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}

pub enum AuthorizeWithTokenError {
    Invalid(ValidateAuthorizeFieldsError),
    DecodeError(DecodeAuthorizeTokenError),
    PermissionError(AuthPermissionError),
}

const ERROR: &'static str = "authorize with token error";

impl std::fmt::Display for AuthorizeWithTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(err) => write!(f, "{}; {}", ERROR, err),
            Self::DecodeError(err) => write!(f, "{}; {}", ERROR, err),
            Self::PermissionError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

impl From<ValidateAuthorizeFieldsError> for AuthorizeWithTokenError {
    fn from(value: ValidateAuthorizeFieldsError) -> Self {
        Self::Invalid(value)
    }
}

impl From<DecodeAuthorizeTokenError> for AuthorizeWithTokenError {
    fn from(value: DecodeAuthorizeTokenError) -> Self {
        Self::DecodeError(value)
    }
}

impl From<AuthPermissionError> for AuthorizeWithTokenError {
    fn from(value: AuthPermissionError) -> Self {
        Self::PermissionError(value)
    }
}

#[derive(Debug)]
pub enum ValidateAuthorizeFieldsError {
    Token(ValidateAuthorizeTokenError),
    Required(ValidateAuthPermissionError),
}

impl std::fmt::Display for ValidateAuthorizeFieldsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Token(err) => write!(f, "invalid token: {}", err),
            Self::Required(err) => write!(f, "invalid permission: {}", err),
        }
    }
}
