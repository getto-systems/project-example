use crate::{
    auth::auth_ticket::_common::kernel::data::DecodeAuthTokenError,
    z_details::_common::{repository::data::RepositoryError, request::data::MetadataError},
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
