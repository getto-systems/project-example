use crate::z_lib::remote::{repository::data::RepositoryError, request::data::MetadataError};

pub enum ValidateAuthNonceError {
    NonceNotSent,
    MetadataError(MetadataError),
    RepositoryError(RepositoryError),
    Conflict,
}

impl std::fmt::Display for ValidateAuthNonceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let label = "auth nonce error";
        match self {
            Self::NonceNotSent => write!(f, "{}: nonce not sent", label),
            Self::MetadataError(err) => write!(f, "{}: {}", label, err),
            Self::RepositoryError(err) => write!(f, "{}: {}", label, err),
            Self::Conflict => write!(f, "{}: conflict", label),
        }
    }
}
