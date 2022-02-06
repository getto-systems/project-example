use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use rand_core::OsRng;

use crate::auth::user::password::kernel::infra::{
    AuthUserPasswordHasher, HashedPassword, PlainPassword,
};

use crate::auth::user::password::kernel::data::PasswordHashError;

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
            .hash_password(self.plain_password.as_bytes(), salt.as_ref())
            .map_err(|err| PasswordHashError::InfraError(format!("{}", err)))?;

        Ok(HashedPassword::restore(hash.to_string()))
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::user::password::kernel::infra::{
        AuthUserPasswordHasher, HashedPassword, PlainPassword,
    };

    use crate::auth::user::password::kernel::data::PasswordHashError;

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
            Ok(HashedPassword::restore(self.password))
        }
    }
}
