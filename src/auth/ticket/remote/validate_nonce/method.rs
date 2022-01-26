use getto_application::data::MethodResult;

use crate::auth::ticket::remote::{
    kernel::{
        data::ExpireDateTime,
        infra::{AuthClock, AuthNonceMetadata},
    },
    validate_nonce::infra::{AuthNonceEntry, AuthNonceRepository},
};

use crate::{
    auth::ticket::remote::kernel::data::ExpireDuration,
    z_lib::remote::{
        repository::data::{RegisterResult, RepositoryError},
        request::data::MetadataError,
    },
};

pub enum ValidateAuthNonceEvent {
    NonceExpiresCalculated(ExpireDateTime),
    Success,
    NonceNotSent,
    Conflict,
    MetadataError(MetadataError),
    RepositoryError(RepositoryError),
}

impl std::fmt::Display for ValidateAuthNonceEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let label = "validate nonce error";
        match self {
            Self::NonceExpiresCalculated(expires) => {
                write!(f, "nonce expires calculated; {}", expires)
            }
            Self::Success => write!(f, "validate nonce success"),
            Self::NonceNotSent => write!(f, "{}; nonce not sent", label),
            Self::MetadataError(err) => write!(f, "{}; {}", label, err),
            Self::RepositoryError(err) => write!(f, "{}; {}", label, err),
            Self::Conflict => write!(f, "{}; conflict", label),
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

    post(ValidateAuthNonceEvent::NonceExpiresCalculated(expires.clone()));

    match nonce_repository
        .put(AuthNonceEntry::new(nonce, expires), registered_at)
        .await
        .map_err(|err| post(ValidateAuthNonceEvent::RepositoryError(err)))?
    {
        RegisterResult::Success(_) => Ok(post(ValidateAuthNonceEvent::Success)),
        RegisterResult::Conflict => Err(post(ValidateAuthNonceEvent::Conflict)),
    }
}
