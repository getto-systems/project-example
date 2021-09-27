use crate::auth::{
    _common::infra::AuthMetadataContent,
    auth_ticket::remote::{
        kernel::infra::{AuthMetadata, AuthTokenDecoder},
        validate_metadata::infra::ValidateAuthMetadataInfra,
    },
};

use crate::auth::auth_ticket::remote::validate_metadata::data::ValidateAuthMetadataError;

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
