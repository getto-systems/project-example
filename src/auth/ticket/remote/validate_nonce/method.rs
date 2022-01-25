use crate::auth::ticket::remote::{
    validate_nonce::infra::{AuthNonceEntry, AuthNonceRepository, ValidateAuthNonceInfra},
    kernel::infra::{AuthClock, AuthNonceMetadata},
};

use crate::{
    auth::ticket::remote::validate_nonce::data::ValidateAuthNonceError,
    z_lib::remote::repository::data::RegisterResult,
};

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
