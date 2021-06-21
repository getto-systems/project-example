use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use rand_core::OsRng;

use super::{AuthUserPasswordHasher, HashedPassword, PlainPassword};

use crate::auth::password::_api::kernel::data::PasswordHashError;

pub struct Argon2PasswordHasher;

impl AuthUserPasswordHasher for Argon2PasswordHasher {
    fn hash_password(
        &self,
        plain_password: PlainPassword,
    ) -> Result<HashedPassword, PasswordHashError> {
        let engine = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);

        let hash = engine
            .hash_password_simple(plain_password.as_bytes(), salt.as_ref())
            .map_err(|err| PasswordHashError::InfraError(format!("{}", err)))?;

        Ok(HashedPassword::new(hash.to_string()))
    }
}

#[cfg(test)]
pub mod test {
    use super::super::{AuthUserPasswordHasher, HashedPassword, PlainPassword};

    use super::super::super::data::PasswordHashError;

    pub struct PlainPasswordHasher;

    impl AuthUserPasswordHasher for PlainPasswordHasher {
        fn hash_password(
            &self,
            plain_password: PlainPassword,
        ) -> Result<HashedPassword, PasswordHashError> {
            Ok(HashedPassword::new(plain_password.extract()))
        }
    }
}
