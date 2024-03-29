use argon2::{Argon2, PasswordHash, PasswordVerifier};

use crate::auth::user::password::kernel::infra::{
    AuthUserPasswordMatcher, HashedPassword, PlainPassword,
};

use crate::auth::user::password::kernel::data::PasswordHashError;

pub struct Argon2PasswordMatcher {
    plain_password: PlainPassword,
}

impl AuthUserPasswordMatcher for Argon2PasswordMatcher {
    fn new(plain_password: PlainPassword) -> Self {
        Self { plain_password }
    }
    fn match_password(self, hashed_password: HashedPassword) -> Result<bool, PasswordHashError> {
        let hashed_password = hashed_password.extract();
        let plain_password = self.plain_password.extract();

        let hash = PasswordHash::new(hashed_password.as_str())
            .map_err(|err| PasswordHashError::InfraError(format!("{}", err)))?;

        Ok(Argon2::default()
            .verify_password(plain_password.as_bytes(), &hash)
            .is_ok())
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::user::password::kernel::infra::{
        AuthUserPasswordMatcher, HashedPassword, PlainPassword,
    };

    use crate::auth::user::password::kernel::data::PasswordHashError;

    pub struct PlainPasswordMatcher {
        plain_password: PlainPassword,
    }

    impl AuthUserPasswordMatcher for PlainPasswordMatcher {
        fn new(plain_password: PlainPassword) -> Self {
            Self { plain_password }
        }
        fn match_password(
            self,
            hashed_password: HashedPassword,
        ) -> Result<bool, PasswordHashError> {
            let password = self.plain_password.extract();
            Ok(password.as_bytes() == hashed_password.extract().as_bytes())
        }
    }
}
