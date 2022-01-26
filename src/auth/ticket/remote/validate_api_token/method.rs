use crate::auth::ticket::remote::{
    kernel::infra::{AuthMetadata, AuthTokenDecoder},
    validate_api_token::infra::ValidateService,
};

use crate::auth::{
    ticket::remote::validate_api_token::data::ValidateApiTokenError,
    user::remote::kernel::data::{AuthUserId, RequireAuthRoles},
};

pub trait ValidateApiTokenInfra {
    type AuthMetadata: AuthMetadata;
    type TokenDecoder: AuthTokenDecoder;
    type ValidateService: ValidateService;

    fn auth_metadata(&self) -> &Self::AuthMetadata;
    fn token_decoder(&self) -> &Self::TokenDecoder;
    fn validate_service(&self) -> &Self::ValidateService;
}

pub async fn validate_api_token(
    infra: &impl ValidateApiTokenInfra,
    require_roles: RequireAuthRoles,
) -> Result<AuthUserId, ValidateApiTokenError> {
    let auth_metadata = infra.auth_metadata();
    let token_decoder = infra.token_decoder();
    let validate_service = infra.validate_service();

    let metadata = auth_metadata
        .metadata()
        .map_err(ValidateApiTokenError::MetadataError)?;

    if let Some(ref token) = metadata.token {
        token_decoder
            .decode(token)
            .map_err(ValidateApiTokenError::DecodeError)?;
    }

    let user_id = validate_service
        .validate(metadata, require_roles)
        .await
        .map_err(ValidateApiTokenError::ServiceError)?;

    Ok(user_id)
}
