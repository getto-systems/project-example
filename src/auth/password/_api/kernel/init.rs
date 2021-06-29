mod password_hasher;
mod password_matcher;
mod password_repository;
mod token_generator;

pub use password_hasher::Argon2PasswordHasher;
pub use password_matcher::Argon2PasswordMatcher;
pub use password_repository::{
    MemoryAuthUserPasswordMap, MemoryAuthUserPasswordRepository, MemoryAuthUserPasswordStore,
};
pub use token_generator::UuidResetTokenGenerator;

#[cfg(test)]
pub mod test {
    pub use super::password_hasher::test::PlainPasswordHasher;
    pub use super::password_matcher::test::PlainPasswordMatcher;
    pub use super::password_repository::{
        MemoryAuthUserPasswordMap, MemoryAuthUserPasswordRepository, MemoryAuthUserPasswordStore,
    };
    pub use super::token_generator::test::StaticResetTokenGenerator;
}
