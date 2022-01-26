use crate::auth::{
    remote::infra::AuthMetadataContent,
    ticket::remote::kernel::infra::{AuthMetadata, AuthTokenDecoder},
};

use crate::auth::ticket::remote::validate_metadata::data::ValidateAuthMetadataError;

pub trait ValidateAuthMetadataInfra {
    type AuthMetadata: AuthMetadata;
    type TokenDecoder: AuthTokenDecoder;

    fn auth_metadata(&self) -> &Self::AuthMetadata;
    fn token_decoder(&self) -> &Self::TokenDecoder;
}

pub fn validate_auth_metadata(
    infra: &impl ValidateAuthMetadataInfra,
) -> Result<AuthMetadataContent, ValidateAuthMetadataError> {
    let auth_metadata = infra.auth_metadata();
    let token_decoder = infra.token_decoder();

    let metadata = auth_metadata
        .metadata()
        .map_err(ValidateAuthMetadataError::MetadataError)?;

    if let Some(ref token) = metadata.token {
        token_decoder
            .decode(token)
            .map_err(ValidateAuthMetadataError::DecodeError)?;
    }

    Ok(metadata)
}
