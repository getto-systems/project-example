use crate::auth::{
    remote::infra::AuthMetadataContent,
    ticket::remote::kernel::infra::{AuthMetadata, AuthTokenDecoder},
};

use crate::{
    auth::ticket::remote::kernel::data::DecodeAuthTokenError,
    z_lib::remote::request::data::MetadataError,
};

pub enum ValidateAuthMetadataEvent {
    Success,
    MetadataError(MetadataError),
    DecodeError(DecodeAuthTokenError),
}

const SUCCESS: &'static str = "validate metadata success";
const ERROR: &'static str = "validate metadata error";

impl std::fmt::Display for ValidateAuthMetadataEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => write!(f, "{}", SUCCESS),
            Self::MetadataError(err) => write!(f, "{}; {}", ERROR, err),
            Self::DecodeError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

pub trait ValidateAuthMetadataInfra {
    type AuthMetadata: AuthMetadata;
    type TokenDecoder: AuthTokenDecoder;

    fn auth_metadata(&self) -> &Self::AuthMetadata;
    fn token_decoder(&self) -> &Self::TokenDecoder;
}

pub fn validate_auth_metadata<S>(
    infra: &impl ValidateAuthMetadataInfra,
    post: impl Fn(ValidateAuthMetadataEvent) -> S,
) -> Result<AuthMetadataContent, S> {
    let auth_metadata = infra.auth_metadata();
    let token_decoder = infra.token_decoder();

    let metadata = auth_metadata
        .metadata()
        .map_err(|err| post(ValidateAuthMetadataEvent::MetadataError(err)))?;

    if let Some(ref token) = metadata.token {
        token_decoder
            .decode(token)
            .map_err(|err| post(ValidateAuthMetadataEvent::DecodeError(err)))?;
    }

    post(ValidateAuthMetadataEvent::Success);
    Ok(metadata)
}
