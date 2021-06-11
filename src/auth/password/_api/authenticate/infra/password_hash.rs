use argon2::{Argon2, PasswordHash, PasswordVerifier};

use super::{AuthUserPasswordHash, HashedPassword, PlainPassword};

use super::super::data::PasswordHashError;

pub struct Argon2PasswordHash;

impl Argon2PasswordHash {
    pub const fn new() -> Self {
        Self
    }
}

impl AuthUserPasswordHash for Argon2PasswordHash {
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

#[cfg(test)]
pub mod test {
    use super::{AuthUserPasswordHash, HashedPassword, PlainPassword};

    use super::super::super::data::PasswordHashError;

    pub struct PlainPasswordHash;

    impl PlainPasswordHash {
        pub const fn new() -> Self {
            Self
        }
    }

    impl AuthUserPasswordHash for PlainPasswordHash {
        fn verify(
            &self,
            plain_password: &PlainPassword,
            hashed_password: &HashedPassword,
        ) -> Result<bool, PasswordHashError> {
            Ok(plain_password.as_bytes() == hashed_password.as_str().as_bytes())
        }
    }
}
