use uuid::Uuid;

use crate::auth::password::reset::remote::request_token::infra::ResetTokenGenerator;

use crate::auth::password::_auth::kernel::data::ResetToken;

pub struct UuidResetTokenGenerator;

impl ResetTokenGenerator for UuidResetTokenGenerator {
    fn generate(&self) -> ResetToken {
        ResetToken::new(Uuid::new_v4().to_string())
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::password::reset::remote::request_token::infra::ResetTokenGenerator;

    use crate::auth::password::_auth::kernel::data::ResetToken;

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
