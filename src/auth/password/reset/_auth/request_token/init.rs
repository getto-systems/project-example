pub(in crate::auth) mod destination_repository;
pub(in crate::auth) mod request_decoder;
pub(in crate::auth) mod token_encoder;
pub(in crate::auth) mod token_generator;
pub(in crate::auth) mod token_notifier;

use tonic::metadata::MetadataMap;

use crate::auth::auth_ticket::_auth::kernel::infra::AuthClockInfra;

use crate::auth::_auth::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::{
    auth_ticket::_auth::kernel::init::{CheckAuthNonceStruct, ChronoAuthClockInitializer},
    password::_auth::kernel::init::AuthUserPasswordStruct,
};
use destination_repository::MysqlResetTokenDestinationRepository;
use token_encoder::JwtResetTokenEncoder;
use token_generator::UuidResetTokenGenerator;
use token_notifier::EmailResetTokenNotifier;

use super::infra::{RequestResetTokenConfig, RequestResetTokenInfra};

pub struct RequestResetTokenStruct<'a> {
    check_nonce_infra: CheckAuthNonceStruct<'a>,
    clock_infra: AuthClockInfra,
    password_infra: AuthUserPasswordStruct<'a>,
    destination_repository: MysqlResetTokenDestinationRepository<'a>,
    token_generator: UuidResetTokenGenerator,
    token_encoder: JwtResetTokenEncoder<'a>,
    token_notifier: EmailResetTokenNotifier<'a>,
    config: RequestResetTokenConfig,
}

impl<'a> RequestResetTokenStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature, metadata: MetadataMap) -> Self {
        Self {
            check_nonce_infra: CheckAuthNonceStruct::new(feature, metadata),
            clock_infra: AuthClockInfra::new(ChronoAuthClockInitializer),
            password_infra: AuthUserPasswordStruct::new(feature),
            destination_repository: MysqlResetTokenDestinationRepository::new(&feature.store.mysql),
            token_generator: UuidResetTokenGenerator,
            token_encoder: JwtResetTokenEncoder::new(&feature.secret),
            token_notifier: EmailResetTokenNotifier::new(&feature.email),
            config: RequestResetTokenConfig {
                token_expires: feature.config.reset_token_expires,
            },
        }
    }
}

impl<'a> RequestResetTokenInfra for RequestResetTokenStruct<'a> {
    type CheckNonceInfra = CheckAuthNonceStruct<'a>;
    type PasswordInfra = AuthUserPasswordStruct<'a>;
    type DestinationRepository = MysqlResetTokenDestinationRepository<'a>;
    type TokenGenerator = UuidResetTokenGenerator;
    type TokenEncoder = JwtResetTokenEncoder<'a>;
    type TokenNotifier = EmailResetTokenNotifier<'a>;

    fn check_nonce_infra(&self) -> &Self::CheckNonceInfra {
        &self.check_nonce_infra
    }
    fn clock_infra(&self) -> &AuthClockInfra {
        &self.clock_infra
    }
    fn password_infra(&self) -> &Self::PasswordInfra {
        &self.password_infra
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
    pub use super::destination_repository::test::{
        MemoryResetTokenDestinationMap, MemoryResetTokenDestinationRepository,
        MemoryResetTokenDestinationStore,
    };
    pub use super::request_decoder::test::StaticRequestResetTokenRequestDecoder;
    pub use super::token_encoder::test::StaticResetTokenEncoder;
    pub use super::token_generator::test::StaticResetTokenGenerator;
    pub use super::token_notifier::test::StaticResetTokenNotifier;

    use crate::auth::{
        auth_ticket::_auth::kernel::init::test::StaticCheckAuthNonceStruct,
        password::_auth::kernel::init::test::StaticAuthUserPasswordStruct,
    };

    use super::super::infra::{RequestResetTokenConfig, RequestResetTokenInfra};
    use crate::auth::auth_ticket::_auth::kernel::infra::AuthClockInfra;

    pub struct StaticRequestResetTokenStruct<'a> {
        pub check_nonce_infra: StaticCheckAuthNonceStruct<'a>,
        pub clock_infra: AuthClockInfra,
        pub password_infra: StaticAuthUserPasswordStruct<'a>,
        pub destination_repository: MemoryResetTokenDestinationRepository<'a>,
        pub token_generator: StaticResetTokenGenerator,
        pub token_encoder: StaticResetTokenEncoder,
        pub token_notifier: StaticResetTokenNotifier,
        pub config: RequestResetTokenConfig,
    }

    impl<'a> RequestResetTokenInfra for StaticRequestResetTokenStruct<'a> {
        type CheckNonceInfra = StaticCheckAuthNonceStruct<'a>;
        type PasswordInfra = StaticAuthUserPasswordStruct<'a>;
        type DestinationRepository = MemoryResetTokenDestinationRepository<'a>;
        type TokenGenerator = StaticResetTokenGenerator;
        type TokenEncoder = StaticResetTokenEncoder;
        type TokenNotifier = StaticResetTokenNotifier;

        fn check_nonce_infra(&self) -> &Self::CheckNonceInfra {
            &self.check_nonce_infra
        }
        fn clock_infra(&self) -> &AuthClockInfra {
            &self.clock_infra
        }
        fn password_infra(&self) -> &Self::PasswordInfra {
            &self.password_infra
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
