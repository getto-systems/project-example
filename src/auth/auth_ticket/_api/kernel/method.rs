use crate::z_details::_api::repository::data::RegisterAttemptResult;

use super::infra::{
    AuthClock, AuthNonceEntry, AuthNonceHeader, AuthNonceRepository, CheckAuthNonceInfra,
};

use super::data::ValidateAuthNonceError;

pub async fn check_nonce(infra: &impl CheckAuthNonceInfra) -> Result<(), ValidateAuthNonceError> {
    let nonce_header = infra.nonce_header();
    let nonce_repository = infra.nonce_repository();
    let clock = infra.clock();
    let config = infra.config();

    let nonce = nonce_header
        .nonce()
        .map_err(ValidateAuthNonceError::HeaderError)?;

    match nonce_repository
        .put(
            AuthNonceEntry::new(nonce, clock.now().expires(&config.nonce_expires)),
            &clock.now(),
        )
        .await
        .map_err(ValidateAuthNonceError::RepositoryError)?
    {
        RegisterAttemptResult::Success(_) => Ok(()),
        RegisterAttemptResult::Conflict => Err(ValidateAuthNonceError::Conflict),
    }
}
