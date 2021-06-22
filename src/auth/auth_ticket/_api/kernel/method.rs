use super::infra::{
    AuthClock, AuthNonceEntry, AuthNonceHeader, AuthNonceRepository, CheckAuthNonceInfra,
};

use super::data::ValidateAuthNonceError;

pub fn check_nonce(infra: &impl CheckAuthNonceInfra) -> Result<(), ValidateAuthNonceError> {
    let nonce_header = infra.nonce_header();
    let nonce_repository = infra.nonce_repository();
    let clock = infra.clock();
    let config = infra.config();

    let nonce = nonce_header
        .nonce()
        .map_err(ValidateAuthNonceError::HeaderError)?;

    let entry = nonce_repository
        .get(&nonce)
        .map_err(ValidateAuthNonceError::RepositoryError)?;

    if let Some(entry) = entry {
        if !entry.has_expired(&clock.now()) {
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
