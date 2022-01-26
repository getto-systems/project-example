use crate::{
    auth::ticket::remote::kernel::data::DecodeAuthTokenError,
    z_lib::remote::request::data::MetadataError,
};

pub enum ValidateAuthTokenError {
    TokenNotSent,
    MetadataError(MetadataError),
    DecodeError(DecodeAuthTokenError),
}

impl std::fmt::Display for ValidateAuthTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let label = "auth token error";
        match self {
            Self::TokenNotSent => write!(f, "{}: token not sent", label),
            Self::MetadataError(err) => write!(f, "{}: {}", label, err),
            Self::DecodeError(err) => write!(f, "{}: {}", label, err),
        }
    }
}
