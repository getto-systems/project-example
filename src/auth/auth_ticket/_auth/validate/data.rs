use crate::z_details::{
    _auth::request::data::MetadataError, _common::repository::data::RepositoryError,
};

pub enum ValidateAuthTokenError {
    TokenNotSent,
    MetadataError(MetadataError),
    DecodeError(DecodeAuthTokenError),
    RepositoryError(RepositoryError),
}

impl std::fmt::Display for ValidateAuthTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let label = "auth token error";
        match self {
            Self::TokenNotSent => write!(f, "{}: token not sent", label),
            Self::MetadataError(err) => write!(f, "{}: {}", label, err),
            Self::DecodeError(err) => write!(f, "{}: {}", label, err),
            Self::RepositoryError(err) => write!(f, "{}: {}", label, err),
        }
    }
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
