use crate::auth::ticket::remote::{
    kernel::infra::{AuthClock, AuthNonceMetadata},
    validate_nonce::infra::{AuthNonceEntry, AuthNonceRepository},
};

use crate::{
    auth::ticket::remote::{
        kernel::data::ExpireDuration, validate_nonce::data::ValidateAuthNonceError,
    },
    z_lib::remote::repository::data::RegisterResult,
};

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

pub async fn validate_auth_nonce(
    infra: &impl ValidateAuthNonceInfra,
) -> Result<(), ValidateAuthNonceError> {
    let clock = infra.clock();
    let nonce_metadata = infra.nonce_metadata();
    let nonce_repository = infra.nonce_repository();
    let config = infra.config();

    let nonce = nonce_metadata
        .nonce()
        .map_err(ValidateAuthNonceError::MetadataError)?
        .ok_or(ValidateAuthNonceError::NonceNotSent)?;

    let registered_at = clock.now();
    let expires = registered_at.clone().expires(&config.nonce_expires);

    match nonce_repository
        .put(AuthNonceEntry::new(nonce, expires), registered_at)
        .await
        .map_err(ValidateAuthNonceError::RepositoryError)?
    {
        RegisterResult::Success(_) => Ok(()),
        RegisterResult::Conflict => Err(ValidateAuthNonceError::Conflict),
    }
}
