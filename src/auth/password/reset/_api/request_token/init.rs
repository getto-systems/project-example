use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::kernel::init::CheckAuthNonceStruct;

use super::infra::{
    destination_repository::MemoryResetTokenDestinationRepository,
    messenger::ProtobufRequestResetTokenMessenger, token_encoder::JwtResetTokenEncoder,
    token_notifier::EmailResetTokenNotifier, RequestResetTokenConfig, RequestResetTokenInfra,
};
use crate::auth::{
    auth_ticket::_api::kernel::infra::clock::ChronoAuthClock,
    password::_api::kernel::infra::{
        password_repository::MemoryAuthUserPasswordRepository,
        token_generator::UuidResetTokenGenerator,
    },
};

pub struct RequestResetTokenStruct<'a> {
    check_nonce_infra: CheckAuthNonceStruct<'a>,
    config: RequestResetTokenConfig,
    clock: ChronoAuthClock,
    destination_repository: MemoryResetTokenDestinationRepository<'a>,
    password_repository: MemoryAuthUserPasswordRepository<'a>,
    token_generator: UuidResetTokenGenerator,
    token_encoder: JwtResetTokenEncoder<'a>,
    token_notifier: EmailResetTokenNotifier<'a>,
    messenger: ProtobufRequestResetTokenMessenger,
}

impl<'a> RequestResetTokenStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature, request: &'a HttpRequest, body: String) -> Self {
        Self {
            check_nonce_infra: CheckAuthNonceStruct::new(feature, request),
            config: RequestResetTokenConfig {
                token_expires: feature.config.reset_token_expires,
            },
            clock: ChronoAuthClock::new(),
            destination_repository: MemoryResetTokenDestinationRepository::new(
                &feature.store.reset_token_destination,
            ),
            password_repository: MemoryAuthUserPasswordRepository::new(&feature.store.user_password),
            token_generator: UuidResetTokenGenerator::new(),
            token_encoder: JwtResetTokenEncoder::new(&feature.secret.reset_token.encoding_key),
            token_notifier: EmailResetTokenNotifier::ap_north_east_1(&feature.email),
            messenger: ProtobufRequestResetTokenMessenger::new(body),
        }
    }
}

impl<'a> RequestResetTokenInfra for RequestResetTokenStruct<'a> {
    type CheckNonceInfra = CheckAuthNonceStruct<'a>;
    type Clock = ChronoAuthClock;
    type DestinationRepository = MemoryResetTokenDestinationRepository<'a>;
    type PasswordRepository = MemoryAuthUserPasswordRepository<'a>;
    type TokenGenerator = UuidResetTokenGenerator;
    type TokenEncoder = JwtResetTokenEncoder<'a>;
    type TokenNotifier = EmailResetTokenNotifier<'a>;
    type Messenger = ProtobufRequestResetTokenMessenger;

    fn check_nonce_infra(&self) -> &Self::CheckNonceInfra {
        &self.check_nonce_infra
    }
    fn config(&self) -> &RequestResetTokenConfig {
        &self.config
    }
    fn clock(&self) -> &Self::Clock {
        &self.clock
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
}

#[cfg(test)]
pub mod test {
    use crate::auth::auth_ticket::_api::kernel::init::test::StaticCheckAuthNonceStruct;

    use super::super::infra::{
        destination_repository::MemoryResetTokenDestinationRepository,
        messenger::test::StaticRequestResetTokenMessenger,
        token_encoder::test::StaticResetTokenEncoder,
        token_notifier::test::StaticResetTokenNotifier, RequestResetTokenConfig,
        RequestResetTokenInfra,
    };
    use crate::auth::{
        auth_ticket::_api::kernel::infra::clock::test::StaticChronoAuthClock,
        password::_api::kernel::infra::{
            password_repository::MemoryAuthUserPasswordRepository,
            token_generator::test::StaticResetTokenGenerator,
        },
    };

    pub struct StaticRequestResetTokenStruct<'a> {
        pub check_nonce_infra: StaticCheckAuthNonceStruct<'a>,
        pub config: RequestResetTokenConfig,
        pub clock: StaticChronoAuthClock,
        pub destination_repository: MemoryResetTokenDestinationRepository<'a>,
        pub password_repository: MemoryAuthUserPasswordRepository<'a>,
        pub token_generator: StaticResetTokenGenerator,
        pub token_encoder: StaticResetTokenEncoder,
        pub token_notifier: StaticResetTokenNotifier,
        pub messenger: StaticRequestResetTokenMessenger,
    }

    impl<'a> RequestResetTokenInfra for StaticRequestResetTokenStruct<'a> {
        type CheckNonceInfra = StaticCheckAuthNonceStruct<'a>;
        type Clock = StaticChronoAuthClock;
        type DestinationRepository = MemoryResetTokenDestinationRepository<'a>;
        type PasswordRepository = MemoryAuthUserPasswordRepository<'a>;
        type TokenGenerator = StaticResetTokenGenerator;
        type TokenEncoder = StaticResetTokenEncoder;
        type TokenNotifier = StaticResetTokenNotifier;
        type Messenger = StaticRequestResetTokenMessenger;

        fn check_nonce_infra(&self) -> &Self::CheckNonceInfra {
            &self.check_nonce_infra
        }
        fn config(&self) -> &RequestResetTokenConfig {
            &self.config
        }
        fn clock(&self) -> &Self::Clock {
            &self.clock
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
    }
}
