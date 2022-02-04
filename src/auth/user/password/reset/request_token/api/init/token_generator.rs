use uuid::Uuid;

use crate::auth::user::password::reset::request_token::api::infra::ResetTokenGenerator;

use crate::auth::user::password::reset::kernel::data::ResetToken;

pub struct UuidResetTokenGenerator;

impl ResetTokenGenerator for UuidResetTokenGenerator {
    fn generate(&self) -> ResetToken {
        ResetToken::new(Uuid::new_v4().to_string())
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::user::password::reset::request_token::api::infra::ResetTokenGenerator;

    use crate::auth::user::password::reset::kernel::data::ResetToken;

    pub struct StaticResetTokenGenerator {
        token: ResetToken,
    }

    impl StaticResetTokenGenerator {
        pub const fn new(token: ResetToken) -> Self {
            Self { token }
        }
    }

    impl ResetTokenGenerator for StaticResetTokenGenerator {
        fn generate(&self) -> ResetToken {
            self.token.clone()
        }
    }
}
