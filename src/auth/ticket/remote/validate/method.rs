use crate::auth::ticket::remote::validate_nonce::method::{
    validate_auth_nonce, ValidateAuthNonceEvent, ValidateAuthNonceInfra,
};

use crate::auth::ticket::remote::{
    kernel::infra::{AuthMetadata, AuthTokenDecoder, AuthTokenMetadata},
    validate::infra::ValidateService,
};

use crate::{
    auth::{
        remote::service::data::AuthServiceError,
        ticket::remote::kernel::data::{AuthTicket, DecodeAuthTokenError, ValidateAuthRolesError},
        user::remote::kernel::data::{AuthUserId, RequireAuthRoles},
    },
    z_lib::remote::request::data::MetadataError,
};

pub enum ValidateAuthTokenEvent {
    ValidateNonce(ValidateAuthNonceEvent),
    Success(AuthTicket),
    TokenNotSent,
    MetadataError(MetadataError),
    DecodeError(DecodeAuthTokenError),
    PermissionError(ValidateAuthRolesError),
}

mod validate_auth_token_event {
    use super::ValidateAuthTokenEvent;

    const SUCCESS: &'static str = "validate success";
    const ERROR: &'static str = "validate error";

    impl std::fmt::Display for ValidateAuthTokenEvent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::ValidateNonce(event) => event.fmt(f),
                Self::Success(ticket) => write!(f, "{}; {}", SUCCESS, ticket),
                Self::TokenNotSent => write!(f, "{}: token not sent", ERROR),
                Self::MetadataError(err) => write!(f, "{}; {}", ERROR, err),
                Self::DecodeError(err) => write!(f, "{}; {}", ERROR, err),
                Self::PermissionError(err) => write!(f, "{}; {}", ERROR, err),
            }
        }
    }
}

pub trait ValidateAuthTokenInfra {
    type ValidateNonce: ValidateAuthNonceInfra;
    type TokenMetadata: AuthTokenMetadata;
    type TokenDecoder: AuthTokenDecoder;

    fn validate_nonce(&self) -> &Self::ValidateNonce;
    fn token_metadata(&self) -> &Self::TokenMetadata;
    fn token_decoder(&self) -> &Self::TokenDecoder;
}

pub async fn validate_auth_token<S>(
    infra: &impl ValidateAuthTokenInfra,
    require_roles: RequireAuthRoles,
    post: impl Fn(ValidateAuthTokenEvent) -> S,
) -> Result<AuthTicket, S> {
    validate_auth_nonce(infra.validate_nonce(), |event| {
        post(ValidateAuthTokenEvent::ValidateNonce(event))
    })
    .await?;

    let ticket = decode_ticket(infra).map_err(|event| post(event))?;

    let ticket = ticket
        .check_enough_permission(require_roles)
        .map_err(|err| post(ValidateAuthTokenEvent::PermissionError(err)))?;

    // 呼び出し側を簡単にするため、例外的に State ではなく AuthTicket を返す
    post(ValidateAuthTokenEvent::Success(ticket.clone()));
    Ok(ticket)
}

fn decode_ticket(
    infra: &impl ValidateAuthTokenInfra,
) -> Result<AuthTicket, ValidateAuthTokenEvent> {
    let token_metadata = infra.token_metadata();
    let token_decoder = infra.token_decoder();

    let token = token_metadata
        .token()
        .map_err(ValidateAuthTokenEvent::MetadataError)?
        .ok_or(ValidateAuthTokenEvent::TokenNotSent)?;

    token_decoder
        .decode(&token)
        .map(|ticket| ticket.restore())
        .map_err(ValidateAuthTokenEvent::DecodeError)
}

pub enum ValidateApiTokenEvent {
    Success(AuthUserId),
    ServiceError(AuthServiceError),
    MetadataError(MetadataError),
    DecodeError(DecodeAuthTokenError),
}

mod validate_api_token_event {
    use super::ValidateApiTokenEvent;

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
