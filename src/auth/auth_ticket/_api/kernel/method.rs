use super::infra::{
    AuthClock, AuthNonceEntry, AuthNonceHeader, AuthNonceRepository, CheckAuthNonceInfra,
};

use super::data::ValidateAuthNonceError;
use crate::z_details::_common::repository::data::RegisterResult;

pub async fn check_nonce(infra: &impl CheckAuthNonceInfra) -> Result<(), ValidateAuthNonceError> {
    let nonce_header = infra.nonce_header();
    let nonce_repository = infra.nonce_repository();
    let clock = infra.clock();
    let config = infra.config();

    let nonce = nonce_header
        .nonce()
        .map_err(ValidateAuthNonceError::HeaderError)?;

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
