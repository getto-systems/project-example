use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use rand_core::OsRng;

use crate::auth::password::_api::kernel::infra::{
    AuthUserPasswordHasher, HashedPassword, PlainPassword,
};

use crate::auth::password::_api::kernel::data::PasswordHashError;

pub struct Argon2PasswordHasher {
    plain_password: PlainPassword,
}

impl AuthUserPasswordHasher for Argon2PasswordHasher {
    fn new(plain_password: PlainPassword) -> Self {
        Self { plain_password }
    }

    fn hash_password(self) -> Result<HashedPassword, PasswordHashError> {
        let engine = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);

        let hash = engine
            .hash_password_simple(self.plain_password.as_bytes(), salt.as_ref())
            .map_err(|err| PasswordHashError::InfraError(format!("{}", err)))?;

        Ok(HashedPassword::new(hash.to_string()))
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::password::_api::kernel::infra::{
        AuthUserPasswordHasher, HashedPassword, PlainPassword,
    };

    use super::super::super::data::PasswordHashError;

    pub struct PlainPasswordHasher {
        password: String,
    }

    impl AuthUserPasswordHasher for PlainPasswordHasher {
        fn new(plain_password: PlainPassword) -> Self {
            Self {
                password: plain_password.extract(),
            }
        }
        fn hash_password(self) -> Result<HashedPassword, PasswordHashError> {
            Ok(HashedPassword::new(self.password))
        }
    }
}
