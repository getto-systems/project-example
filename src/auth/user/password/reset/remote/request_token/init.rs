pub(in crate::auth) mod destination_repository;
pub(in crate::auth) mod request_decoder;
pub(in crate::auth) mod token_encoder;
pub(in crate::auth) mod token_generator;
pub(in crate::auth) mod token_notifier;

use tonic::metadata::MetadataMap;

use crate::auth::remote::x_outside_feature::auth::feature::AuthOutsideFeature;

use crate::auth::{
    ticket::remote::{
        check_nonce::init::CheckAuthNonceStruct, kernel::init::clock::ChronoAuthClock,
    },
    user::password::remote::kernel::init::password_repository::MysqlAuthUserPasswordRepository,
};
use destination_repository::MysqlResetTokenDestinationRepository;
use token_encoder::JwtResetTokenEncoder;
use token_generator::UuidResetTokenGenerator;
use token_notifier::EmailResetTokenNotifier;

use super::infra::{RequestResetTokenConfig, RequestResetTokenInfra};

pub struct RequestResetTokenStruct<'a> {
    check_nonce_infra: CheckAuthNonceStruct<'a>,
    clock: ChronoAuthClock,
    password_repository: MysqlAuthUserPasswordRepository<'a>,
    destination_repository: MysqlResetTokenDestinationRepository<'a>,
    token_generator: UuidResetTokenGenerator,
    token_encoder: JwtResetTokenEncoder<'a>,
    token_notifier: EmailResetTokenNotifier<'a>,
    config: RequestResetTokenConfig,
}

impl<'a> RequestResetTokenStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature, metadata: &'a MetadataMap) -> Self {
        Self {
            check_nonce_infra: CheckAuthNonceStruct::new(feature, metadata),
            clock: ChronoAuthClock::new(),
            password_repository: MysqlAuthUserPasswordRepository::new(&feature.store.mysql),
            destination_repository: MysqlResetTokenDestinationRepository::new(&feature.store.mysql),
            token_generator: UuidResetTokenGenerator,
            token_encoder: JwtResetTokenEncoder::new(&feature.reset_token_key),
            token_notifier: EmailResetTokenNotifier::new(&feature.email),
            config: RequestResetTokenConfig {
                token_expires: feature.config.reset_token_expires,
            },
        }
    }
}

impl<'a> RequestResetTokenInfra for RequestResetTokenStruct<'a> {
    type CheckNonceInfra = CheckAuthNonceStruct<'a>;
    type Clock = ChronoAuthClock;
    type PasswordRepository = MysqlAuthUserPasswordRepository<'a>;
    type DestinationRepository = MysqlResetTokenDestinationRepository<'a>;
    type TokenGenerator = UuidResetTokenGenerator;
    type TokenEncoder = JwtResetTokenEncoder<'a>;
    type TokenNotifier = EmailResetTokenNotifier<'a>;

    fn check_nonce_infra(&self) -> &Self::CheckNonceInfra {
        &self.check_nonce_infra
    }
    fn clock(&self) -> &Self::Clock {
        &self.clock
    }
    fn password_repository(&self) -> &Self::PasswordRepository {
        &self.password_repository
    }
    fn destination_repository(&self) -> &Self::DestinationRepository {
        &self.destination_repository
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
    fn config(&self) -> &RequestResetTokenConfig {
        &self.config
    }
}

#[cfg(test)]
pub mod test {
    use super::destination_repository::test::MemoryResetTokenDestinationRepository;
    use super::token_encoder::test::StaticResetTokenEncoder;
    use super::token_generator::test::StaticResetTokenGenerator;
    use super::token_notifier::test::StaticResetTokenNotifier;

    use crate::auth::{
        ticket::remote::{
            check_nonce::init::test::StaticCheckAuthNonceStruct,
            kernel::init::clock::test::StaticChronoAuthClock,
        },
        user::password::remote::kernel::init::password_repository::test::MemoryAuthUserPasswordRepository,
    };

    use super::super::infra::{RequestResetTokenConfig, RequestResetTokenInfra};

    pub struct StaticRequestResetTokenStruct<'a> {
        pub check_nonce_infra: StaticCheckAuthNonceStruct<'a>,
        pub clock: StaticChronoAuthClock,
        pub password_repository: MemoryAuthUserPasswordRepository<'a>,
        pub destination_repository: MemoryResetTokenDestinationRepository<'a>,
        pub token_generator: StaticResetTokenGenerator,
        pub token_encoder: StaticResetTokenEncoder,
        pub token_notifier: StaticResetTokenNotifier,
        pub config: RequestResetTokenConfig,
    }

    impl<'a> RequestResetTokenInfra for StaticRequestResetTokenStruct<'a> {
        type CheckNonceInfra = StaticCheckAuthNonceStruct<'a>;
        type Clock = StaticChronoAuthClock;
        type PasswordRepository = MemoryAuthUserPasswordRepository<'a>;
        type DestinationRepository = MemoryResetTokenDestinationRepository<'a>;
        type TokenGenerator = StaticResetTokenGenerator;
        type TokenEncoder = StaticResetTokenEncoder;
        type TokenNotifier = StaticResetTokenNotifier;

        fn check_nonce_infra(&self) -> &Self::CheckNonceInfra {
            &self.check_nonce_infra
        }
        fn clock(&self) -> &Self::Clock {
            &self.clock
        }
        fn password_repository(&self) -> &Self::PasswordRepository {
            &self.password_repository
        }
        fn destination_repository(&self) -> &Self::DestinationRepository {
            &self.destination_repository
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
        fn config(&self) -> &RequestResetTokenConfig {
            &self.config
        }
    }
}
