use argon2::{Argon2, PasswordHash, PasswordVerifier};

use super::{AuthUserPasswordMatcher, HashedPassword, PlainPassword};

use super::super::data::PasswordHashError;

pub struct Argon2PasswordMatcher {
    plain_password: PlainPassword,
}

impl AuthUserPasswordMatcher for Argon2PasswordMatcher {
    fn new(plain_password: PlainPassword) -> Self {
        Self { plain_password }
    }
    fn match_password(&self, hashed_password: &HashedPassword) -> Result<bool, PasswordHashError> {
        let engine = Argon2::default();

        let hash = PasswordHash::new(hashed_password.as_str())
            .map_err(|err| PasswordHashError::InfraError(format!("{}", err)))?;

        Ok(engine
            .verify_password(self.plain_password.as_bytes(), &hash)
            .is_ok())
    }
}

#[cfg(test)]
pub mod test {
    use super::{AuthUserPasswordMatcher, HashedPassword, PlainPassword};

    use super::super::super::data::PasswordHashError;

    pub struct PlainPasswordMatcher {
        plain_password: PlainPassword,
    }

    impl AuthUserPasswordMatcher for PlainPasswordMatcher {
        fn new(plain_password: PlainPassword) -> Self {
            Self { plain_password }
        }
        fn match_password(
            &self,
            hashed_password: &HashedPassword,
        ) -> Result<bool, PasswordHashError> {
            Ok(self.plain_password.as_bytes() == hashed_password.as_str().as_bytes())
        }
    }
}
