use uuid::Uuid;

use crate::auth::user::account::register::infra::AuthUserIdGenerator;

use crate::auth::user::kernel::data::AuthUserId;

pub struct UuidAuthUserIdGenerator;

impl UuidAuthUserIdGenerator {
    pub const fn new() -> Self {
        Self
    }
}

impl AuthUserIdGenerator for UuidAuthUserIdGenerator {
    fn generate(&self) -> AuthUserId {
        AuthUserId::restore(Uuid::new_v4().to_string())
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::user::account::register::infra::AuthUserIdGenerator;

    use crate::auth::user::kernel::data::AuthUserId;

    pub struct StaticAuthUserIdGenerator {
        user_id: AuthUserId,
    }

    impl StaticAuthUserIdGenerator {
        pub const fn new(user_id: AuthUserId) -> Self {
            Self { user_id }
        }
    }

    impl AuthUserIdGenerator for StaticAuthUserIdGenerator {
        fn generate(&self) -> AuthUserId {
            self.user_id.clone()
        }
    }
}
