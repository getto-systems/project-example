use crate::{
    auth::{
        remote::service::data::AuthServiceError,
        ticket::remote::{
            kernel::{
                data::DecodeAuthTokenError,
                infra::{AuthMetadata, AuthTokenDecoder},
            },
            validate_api_token::infra::ValidateService,
        },
    },
    z_lib::remote::request::data::MetadataError,
};

use crate::auth::user::remote::kernel::data::{AuthUserId, RequireAuthRoles};

pub enum ValidateApiTokenEvent {
    Success(AuthUserId),
    ServiceError(AuthServiceError),
    MetadataError(MetadataError),
    DecodeError(DecodeAuthTokenError),
}

const SUCCESS: &'static str = "validate api token success";
const ERROR: &'static str = "validate api token error";

impl std::fmt::Display for ValidateApiTokenEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success(user_id) => write!(f, "{}; {}", SUCCESS, user_id),
            Self::ServiceError(err) => write!(f, "{}; {}", ERROR, err),
            Self::MetadataError(err) => write!(f, "{}; {}", ERROR, err),
            Self::DecodeError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

pub trait ValidateApiTokenInfra {
    type AuthMetadata: AuthMetadata;
    type TokenDecoder: AuthTokenDecoder;
    type ValidateService: ValidateService;

    fn auth_metadata(&self) -> &Self::AuthMetadata;
    fn token_decoder(&self) -> &Self::TokenDecoder;
    fn validate_service(&self) -> &Self::ValidateService;
}

pub async fn validate_api_token<S>(
    infra: &impl ValidateApiTokenInfra,
    require_roles: RequireAuthRoles,
    post: impl Fn(ValidateApiTokenEvent) -> S,
) -> Result<AuthUserId, S> {
    let auth_metadata = infra.auth_metadata();
    let token_decoder = infra.token_decoder();
    let validate_service = infra.validate_service();

    let metadata = auth_metadata
        .metadata()
        .map_err(|err| post(ValidateApiTokenEvent::MetadataError(err)))?;

    if let Some(ref token) = metadata.token {
        token_decoder
            .decode(token)
            .map_err(|err| post(ValidateApiTokenEvent::DecodeError(err)))?;
    }

    let user_id = validate_service
        .validate(metadata, require_roles)
        .await
        .map_err(|err| post(ValidateApiTokenEvent::ServiceError(err)))?;

    post(ValidateApiTokenEvent::Success(user_id.clone()));
    Ok(user_id)
}
