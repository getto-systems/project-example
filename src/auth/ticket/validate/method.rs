use getto_application::data::MethodResult;

use crate::auth::ticket::{
    kernel::remote::infra::AuthClock,
    validate::infra::{
        AuthMetadata, AuthMetadataContent, AuthNonceEntry, AuthNonceMetadata, AuthNonceRepository,
        AuthTokenDecoder, AuthTokenMetadata, ValidateService,
    },
};

use crate::{
    auth::{
        remote::proxy::data::AuthProxyError,
        ticket::kernel::remote::data::{
            AuthTicket, DecodeAuthTokenError, ExpireDateTime, ExpireDuration,
            ValidateAuthRolesError,
        },
        user::remote::kernel::data::RequireAuthRoles,
    },
    z_lib::remote::{
        repository::data::{RegisterResult, RepositoryError},
        request::data::MetadataError,
    },
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

pub enum CheckPermissionEvent {
    Success,
    ServiceError(AuthProxyError),
    MetadataError(MetadataError),
    DecodeError(DecodeAuthTokenError),
}

mod check_permission_event {
    use super::CheckPermissionEvent;

    const SUCCESS: &'static str = "validate api token success";
    const ERROR: &'static str = "validate api token error";

    impl std::fmt::Display for CheckPermissionEvent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Success => write!(f, "{}", SUCCESS),
                Self::ServiceError(err) => write!(f, "{}; {}", ERROR, err),
                Self::MetadataError(err) => write!(f, "{}; {}", ERROR, err),
                Self::DecodeError(err) => write!(f, "{}; {}", ERROR, err),
            }
        }
    }
}

pub trait CheckPermissionInfra {
    type AuthMetadata: AuthMetadata;
    type TokenDecoder: AuthTokenDecoder;
    type ValidateService: ValidateService;

    fn auth_metadata(&self) -> &Self::AuthMetadata;
    fn token_decoder(&self) -> &Self::TokenDecoder;
    fn validate_service(&self) -> &Self::ValidateService;
}

pub async fn check_permission<S>(
    infra: &impl CheckPermissionInfra,
    require_roles: RequireAuthRoles,
    post: impl Fn(CheckPermissionEvent) -> S,
) -> MethodResult<S> {
    let auth_metadata = infra.auth_metadata();
    let token_decoder = infra.token_decoder();
    let validate_service = infra.validate_service();

    let metadata = auth_metadata
        .metadata()
        .map_err(|err| post(CheckPermissionEvent::MetadataError(err)))?;

    if let Some(ref token) = metadata.token {
        token_decoder
            .decode(token)
            .map_err(|err| post(CheckPermissionEvent::DecodeError(err)))?;
    }

    validate_service
        .validate(metadata, require_roles)
        .await
        .map_err(|err| post(CheckPermissionEvent::ServiceError(err)))?;

    Ok(post(CheckPermissionEvent::Success))
}

pub enum ValidateAuthMetadataEvent {
    Success,
    MetadataError(MetadataError),
    DecodeError(DecodeAuthTokenError),
}

mod validate_auth_metadata_event {
    use super::ValidateAuthMetadataEvent;

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

pub enum ValidateAuthNonceEvent {
    NonceExpiresCalculated(ExpireDateTime),
    Success,
    NonceNotSent,
    Conflict,
    MetadataError(MetadataError),
    RepositoryError(RepositoryError),
}

mod validate_auth_nonce_event {
    use super::ValidateAuthNonceEvent;

    const SUCCESS: &'static str = "validate nonce success";
    const ERROR: &'static str = "validate nonce error";

    impl std::fmt::Display for ValidateAuthNonceEvent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            match self {
                Self::NonceExpiresCalculated(expires) => {
                    write!(f, "nonce expires calculated; {}", expires)
                }
                Self::Success => write!(f, "{}", SUCCESS),
                Self::NonceNotSent => write!(f, "{}; nonce not sent", ERROR),
                Self::MetadataError(err) => write!(f, "{}; {}", ERROR, err),
                Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
                Self::Conflict => write!(f, "{}; conflict", ERROR),
            }
        }
    }
}

pub trait ValidateAuthNonceInfra {
    type Clock: AuthClock;
    type NonceMetadata: AuthNonceMetadata;
    type NonceRepository: AuthNonceRepository;

    fn clock(&self) -> &Self::Clock;
    fn nonce_metadata(&self) -> &Self::NonceMetadata;
    fn nonce_repository(&self) -> &Self::NonceRepository;
    fn config(&self) -> &AuthNonceConfig;
}

pub struct AuthNonceConfig {
    pub nonce_expires: ExpireDuration,
}

pub async fn validate_auth_nonce<S>(
    infra: &impl ValidateAuthNonceInfra,
    post: impl Fn(ValidateAuthNonceEvent) -> S,
) -> MethodResult<S> {
    let clock = infra.clock();
    let nonce_metadata = infra.nonce_metadata();
    let nonce_repository = infra.nonce_repository();
    let config = infra.config();

    let nonce = nonce_metadata
        .nonce()
        .map_err(|err| post(ValidateAuthNonceEvent::MetadataError(err)))?
        .ok_or_else(|| post(ValidateAuthNonceEvent::NonceNotSent))?;

    let registered_at = clock.now();
    let expires = registered_at.clone().expires(&config.nonce_expires);

    post(ValidateAuthNonceEvent::NonceExpiresCalculated(
        expires.clone(),
    ));

    match nonce_repository
        .put(AuthNonceEntry::new(nonce, expires), registered_at)
        .await
        .map_err(|err| post(ValidateAuthNonceEvent::RepositoryError(err)))?
    {
        RegisterResult::Success(_) => Ok(post(ValidateAuthNonceEvent::Success)),
        RegisterResult::Conflict => Err(post(ValidateAuthNonceEvent::Conflict)),
    }
}
