use argon2::{Argon2, PasswordHash, PasswordVerifier};

use super::{AuthUserPasswordHash, HashedPassword, PlainPassword};

use super::super::data::PasswordHashError;

pub struct PassthroughPasswordHash {}

impl PassthroughPasswordHash {
    pub const fn new() -> Self {
        Self {}
    }
}

impl AuthUserPasswordHash for PassthroughPasswordHash {
    fn verify(
        &self,
        plain_password: &PlainPassword,
        hashed_password: &HashedPassword,
    ) -> Result<bool, PasswordHashError> {
        let engine = Argon2::default();

        let hash = PasswordHash::new(hashed_password.as_str())
            .map_err(|err| PasswordHashError::InfraError(format!("{}", err)))?;

        Ok(engine
            .verify_password(plain_password.as_bytes(), &hash)
            .is_ok())
    }
}
