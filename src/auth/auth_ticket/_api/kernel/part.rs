use super::infra::{AuthClock, AuthNonceConfig, AuthNonceEntry, AuthNonceHeader, AuthNonceRepository};

use super::data::ValidateAuthNonceError;

pub fn check_nonce(
    config: &AuthNonceConfig,
    clock: &impl AuthClock,
    header: &impl AuthNonceHeader,
    nonce_repository: &impl AuthNonceRepository,
) -> Result<(), ValidateAuthNonceError> {
    let nonce = header
        .nonce()
        .map_err(ValidateAuthNonceError::HeaderError)?;

    let entry = nonce_repository
        .get(&nonce)
        .map_err(ValidateAuthNonceError::RepositoryError)?;

    if let Some(entry) = entry {
        if !entry.has_elapsed(clock.now()) {
            return Err(ValidateAuthNonceError::Conflict);
        }
    }

    nonce_repository
        .put(AuthNonceEntry::new(
            nonce,
            clock.now().expires(&config.nonce_expires),
        ))
        .map_err(ValidateAuthNonceError::RepositoryError)?;

    Ok(())
}
