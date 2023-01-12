mod request;
mod token_generator;
mod token_notifier;

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::auth::{
    kernel::init::clock::ChronoAuthClock,
    user::{
        kernel::init::user_repository::dynamodb::DynamoDbAuthUserRepository,
        password::reset::{
            kernel::init::token::encoder::JwtResetPasswordTokenEncoder,
            request_token::init::{
                token_generator::UuidResetTokenGenerator, token_notifier::EmailResetTokenNotifier,
            },
        },
    },
};

use super::action::{RequestResetTokenAction, RequestResetTokenMaterial};

use crate::auth::user::password::reset::request_token::infra::RequestResetPasswordTokenConfig;

pub struct ActiveRequestResetTokenMaterial<'a> {
    clock: ChronoAuthClock,
    reset_token_repository: DynamoDbAuthUserRepository<'a>,
    token_generator: UuidResetTokenGenerator,
    token_encoder: JwtResetPasswordTokenEncoder<'a>,
    token_notifier: EmailResetTokenNotifier<'a>,
    config: RequestResetPasswordTokenConfig,
}

impl<'a> ActiveRequestResetTokenMaterial<'a> {
    pub fn action(feature: &'a AuthAppFeature) -> RequestResetTokenAction<Self> {
        RequestResetTokenAction::with_material(Self {
            clock: ChronoAuthClock::new(),
            reset_token_repository: DynamoDbAuthUserRepository::new(&feature.store),
            token_generator: UuidResetTokenGenerator,
            token_encoder: JwtResetPasswordTokenEncoder::new(&feature.reset_token_key),
            token_notifier: EmailResetTokenNotifier::new(&feature.email),
            config: RequestResetPasswordTokenConfig {
                token_expires: feature.config.reset_token_expires,
            },
        })
    }
}

impl<'a> RequestResetTokenMaterial for ActiveRequestResetTokenMaterial<'a> {
    type Clock = ChronoAuthClock;
    type ResetTokenRepository = DynamoDbAuthUserRepository<'a>;
    type IdGenerator = UuidResetTokenGenerator;
    type TokenEncoder = JwtResetPasswordTokenEncoder<'a>;
    type TokenNotifier = EmailResetTokenNotifier<'a>;

    fn clock(&self) -> &Self::Clock {
        &self.clock
    }
    fn reset_token_repository(&self) -> &Self::ResetTokenRepository {
        &self.reset_token_repository
    }
    fn id_generator(&self) -> &Self::IdGenerator {
        &self.token_generator
    }
    fn token_encoder(&self) -> &Self::TokenEncoder {
        &self.token_encoder
    }
    fn token_notifier(&self) -> &Self::TokenNotifier {
        &self.token_notifier
    }
    fn config(&self) -> &RequestResetPasswordTokenConfig {
        &self.config
    }
}

#[cfg(test)]
pub mod test {
    pub use super::request::test::*;
    pub use super::token_generator::test::*;
    pub use super::token_notifier::test::*;

    use crate::auth::{
        kernel::init::clock::test::StaticChronoAuthClock,
        user::{
            kernel::init::user_repository::memory::MemoryAuthUserRepository,
            password::reset::kernel::init::token::encoder::test::StaticResetTokenEncoder,
        },
    };

    use crate::auth::user::password::reset::request_token::action::RequestResetTokenMaterial;

    use crate::auth::user::password::reset::request_token::infra::RequestResetPasswordTokenConfig;

    pub struct StaticRequestResetTokenMaterial<'a> {
        clock: StaticChronoAuthClock,
        reset_token_repository: MemoryAuthUserRepository<'a>,
        token_generator: StaticResetTokenGenerator,
        token_encoder: StaticResetTokenEncoder,
        token_notifier: StaticResetTokenNotifier,
        config: RequestResetPasswordTokenConfig,
    }

    impl<'a> StaticRequestResetTokenMaterial<'a> {
        pub fn new(
            clock: StaticChronoAuthClock,
            reset_token_repository: MemoryAuthUserRepository<'a>,
            token_generator: StaticResetTokenGenerator,
            config: RequestResetPasswordTokenConfig,
        ) -> Self {
            Self {
                clock,
                reset_token_repository,
                token_generator,
                token_encoder: StaticResetTokenEncoder,
                token_notifier: StaticResetTokenNotifier,
                config,
            }
        }
    }

    impl<'a> RequestResetTokenMaterial for StaticRequestResetTokenMaterial<'a> {
        type Clock = StaticChronoAuthClock;
        type ResetTokenRepository = MemoryAuthUserRepository<'a>;
        type IdGenerator = StaticResetTokenGenerator;
        type TokenEncoder = StaticResetTokenEncoder;
        type TokenNotifier = StaticResetTokenNotifier;

        fn clock(&self) -> &Self::Clock {
            &self.clock
        }
        fn reset_token_repository(&self) -> &Self::ResetTokenRepository {
            &self.reset_token_repository
        }
        fn id_generator(&self) -> &Self::IdGenerator {
            &self.token_generator
        }
        fn token_encoder(&self) -> &Self::TokenEncoder {
            &self.token_encoder
        }
        fn token_notifier(&self) -> &Self::TokenNotifier {
            &self.token_notifier
        }
        fn config(&self) -> &RequestResetPasswordTokenConfig {
            &self.config
        }
    }
}
