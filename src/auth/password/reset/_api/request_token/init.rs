mod destination_repository;
mod messenger;
mod token_encoder;
mod token_notifier;

use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::kernel::init::AuthClockStruct;
use crate::auth::{
    auth_ticket::_api::kernel::init::CheckAuthNonceStruct,
    password::_api::kernel::init::{MemoryAuthUserPasswordRepository, UuidResetTokenGenerator},
};
pub use destination_repository::{
    MemoryResetTokenDestinationMap, MemoryResetTokenDestinationRepository,
    MemoryResetTokenDestinationStore,
};
use messenger::ProtobufRequestResetTokenMessenger;
use token_encoder::JwtResetTokenEncoder;
use token_notifier::EmailResetTokenNotifier;

use super::infra::{RequestResetTokenConfig, RequestResetTokenInfra};

pub struct RequestResetTokenStruct<'a> {
    check_nonce_infra: CheckAuthNonceStruct<'a>,
    clock_infra: AuthClockStruct,
    destination_repository: MemoryResetTokenDestinationRepository<'a>,
    password_repository: MemoryAuthUserPasswordRepository<'a>,
    token_generator: UuidResetTokenGenerator,
    token_encoder: JwtResetTokenEncoder<'a>,
    token_notifier: EmailResetTokenNotifier<'a>,
    messenger: ProtobufRequestResetTokenMessenger,
    config: RequestResetTokenConfig,
}

impl<'a> RequestResetTokenStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature, request: &'a HttpRequest, body: String) -> Self {
        Self {
            check_nonce_infra: CheckAuthNonceStruct::new(feature, request),
            clock_infra: AuthClockStruct::new(),
            destination_repository: MemoryResetTokenDestinationRepository::new(
                &feature.store.reset_token_destination,
            ),
            password_repository: MemoryAuthUserPasswordRepository::new(
                &feature.store.user_password,
            ),
            token_generator: UuidResetTokenGenerator::new(),
            token_encoder: JwtResetTokenEncoder::new(&feature.secret.reset_token.encoding_key),
            token_notifier: EmailResetTokenNotifier::ap_north_east_1(&feature.email),
            messenger: ProtobufRequestResetTokenMessenger::new(body),
            config: RequestResetTokenConfig {
                token_expires: feature.config.reset_token_expires,
            },
        }
    }
}

impl<'a> RequestResetTokenInfra for RequestResetTokenStruct<'a> {
    type CheckNonceInfra = CheckAuthNonceStruct<'a>;
    type ClockInfra = AuthClockStruct;
    type DestinationRepository = MemoryResetTokenDestinationRepository<'a>;
    type PasswordRepository = MemoryAuthUserPasswordRepository<'a>;
    type TokenGenerator = UuidResetTokenGenerator;
    type TokenEncoder = JwtResetTokenEncoder<'a>;
    type TokenNotifier = EmailResetTokenNotifier<'a>;
    type Messenger = ProtobufRequestResetTokenMessenger;

    fn check_nonce_infra(&self) -> &Self::CheckNonceInfra {
        &self.check_nonce_infra
    }
    fn clock_infra(&self) -> &Self::ClockInfra {
        &self.clock_infra
    }
    fn destination_repository(&self) -> &Self::DestinationRepository {
        &self.destination_repository
    }
    fn password_repository(&self) -> &Self::PasswordRepository {
        &self.password_repository
    }
    fn token_generator(&self) -> &Self::TokenGenerator {
        &self.token_generator
    }
    fn token_encoder(&self) -> &Self::TokenEncoder {
        &self.token_encoder
    }
    fn token_notifier(&self) -> &Self::TokenNotifier {
        &self.token_notifier
    }
    fn messenger(&self) -> &Self::Messenger {
        &self.messenger
    }
    fn config(&self) -> &RequestResetTokenConfig {
        &self.config
    }
}

#[cfg(test)]
pub mod test {
    pub use super::destination_repository::{
        MemoryResetTokenDestinationMap, MemoryResetTokenDestinationRepository,
        MemoryResetTokenDestinationStore,
    };
    pub use super::messenger::test::StaticRequestResetTokenMessenger;
    pub use super::token_encoder::test::StaticResetTokenEncoder;
    pub use super::token_notifier::test::StaticResetTokenNotifier;
    use crate::auth::auth_ticket::_api::kernel::init::test::StaticAuthClockStruct;
    use crate::auth::{
        auth_ticket::_api::kernel::init::test::StaticCheckAuthNonceStruct,
        password::_api::kernel::init::test::{
            MemoryAuthUserPasswordRepository, StaticResetTokenGenerator,
        },
    };

    use super::super::infra::{RequestResetTokenConfig, RequestResetTokenInfra};

    pub struct StaticRequestResetTokenStruct<'a> {
        pub check_nonce_infra: StaticCheckAuthNonceStruct<'a>,
        pub clock_infra: StaticAuthClockStruct,
        pub destination_repository: MemoryResetTokenDestinationRepository<'a>,
        pub password_repository: MemoryAuthUserPasswordRepository<'a>,
        pub token_generator: StaticResetTokenGenerator,
        pub token_encoder: StaticResetTokenEncoder,
        pub token_notifier: StaticResetTokenNotifier,
        pub messenger: StaticRequestResetTokenMessenger,
        pub config: RequestResetTokenConfig,
    }

    impl<'a> RequestResetTokenInfra for StaticRequestResetTokenStruct<'a> {
        type CheckNonceInfra = StaticCheckAuthNonceStruct<'a>;
        type ClockInfra = StaticAuthClockStruct;
        type DestinationRepository = MemoryResetTokenDestinationRepository<'a>;
        type PasswordRepository = MemoryAuthUserPasswordRepository<'a>;
        type TokenGenerator = StaticResetTokenGenerator;
        type TokenEncoder = StaticResetTokenEncoder;
        type TokenNotifier = StaticResetTokenNotifier;
        type Messenger = StaticRequestResetTokenMessenger;

        fn check_nonce_infra(&self) -> &Self::CheckNonceInfra {
            &self.check_nonce_infra
        }
        fn clock_infra(&self) -> &Self::ClockInfra {
            &self.clock_infra
        }
        fn destination_repository(&self) -> &Self::DestinationRepository {
            &self.destination_repository
        }
        fn password_repository(&self) -> &Self::PasswordRepository {
            &self.password_repository
        }
        fn token_generator(&self) -> &Self::TokenGenerator {
            &self.token_generator
        }
        fn token_encoder(&self) -> &Self::TokenEncoder {
            &self.token_encoder
        }
        fn token_notifier(&self) -> &Self::TokenNotifier {
            &self.token_notifier
        }
        fn messenger(&self) -> &Self::Messenger {
            &self.messenger
        }
        fn config(&self) -> &RequestResetTokenConfig {
            &self.config
        }
    }
}
