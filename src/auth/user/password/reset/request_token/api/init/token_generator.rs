use uuid::Uuid;

use crate::auth::user::password::reset::request_token::infra::ResetPasswordIdGenerator;

use crate::auth::user::password::reset::kernel::data::ResetPasswordId;

pub struct UuidResetTokenGenerator;

impl ResetPasswordIdGenerator for UuidResetTokenGenerator {
    fn generate(&self) -> ResetPasswordId {
        ResetPasswordId::restore(Uuid::new_v4().to_string())
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::user::password::reset::request_token::infra::ResetPasswordIdGenerator;

    use crate::auth::user::password::reset::kernel::data::ResetPasswordId;

    pub struct StaticResetTokenGenerator {
        token: ResetPasswordId,
    }

    impl StaticResetTokenGenerator {
        pub const fn new(token: ResetPasswordId) -> Self {
            Self { token }
        }
    }

    impl ResetPasswordIdGenerator for StaticResetTokenGenerator {
        fn generate(&self) -> ResetPasswordId {
            self.token.clone()
        }
    }
}
