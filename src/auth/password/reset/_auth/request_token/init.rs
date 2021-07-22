mod destination_repository;
mod request_decoder;
mod token_encoder;
mod token_generator;
mod token_notifier;

use tonic::metadata::MetadataMap;

use crate::auth::password::reset::_common::y_protobuf::service::RequestResetTokenRequestPb;

use crate::auth::_auth::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::{
    auth_ticket::_auth::kernel::init::{AuthClockStruct, CheckAuthNonceStruct},
    password::_auth::kernel::init::AuthUserPasswordStruct,
};
use destination_repository::MysqlResetTokenDestinationRepository;
use request_decoder::PbRequestResetTokenRequestDecoder;
use token_encoder::JwtResetTokenEncoder;
use token_generator::UuidResetTokenGenerator;
use token_notifier::EmailResetTokenNotifier;

use super::infra::{RequestResetTokenConfig, RequestResetTokenInfra};

pub struct RequestResetTokenStruct<'a> {
    check_nonce_infra: CheckAuthNonceStruct<'a>,
    clock_infra: AuthClockStruct,
    password_infra: AuthUserPasswordStruct<'a>,
    destination_repository: MysqlResetTokenDestinationRepository<'a>,
    token_generator: UuidResetTokenGenerator,
    token_encoder: JwtResetTokenEncoder<'a>,
    token_notifier: EmailResetTokenNotifier<'a>,
    request_decoder: PbRequestResetTokenRequestDecoder,
    config: RequestResetTokenConfig,
}

impl<'a> RequestResetTokenStruct<'a> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        metadata: MetadataMap,
        request: RequestResetTokenRequestPb,
    ) -> Self {
        Self {
            check_nonce_infra: CheckAuthNonceStruct::new(feature, metadata),
            clock_infra: AuthClockStruct::new(),
            password_infra: AuthUserPasswordStruct::new(feature),
            destination_repository: MysqlResetTokenDestinationRepository::new(&feature.store.mysql),
            token_generator: UuidResetTokenGenerator,
            token_encoder: JwtResetTokenEncoder::new(&feature.secret),
            token_notifier: EmailResetTokenNotifier::new(&feature.email),
            request_decoder: PbRequestResetTokenRequestDecoder::new(request),
            config: RequestResetTokenConfig {
                token_expires: feature.config.reset_token_expires,
            },
        }
    }
}

impl<'a> RequestResetTokenInfra for RequestResetTokenStruct<'a> {
    type CheckNonceInfra = CheckAuthNonceStruct<'a>;
    type ClockInfra = AuthClockStruct;
    type PasswordInfra = AuthUserPasswordStruct<'a>;
    type RequestDecoder = PbRequestResetTokenRequestDecoder;
    type DestinationRepository = MysqlResetTokenDestinationRepository<'a>;
    type TokenGenerator = UuidResetTokenGenerator;
    type TokenEncoder = JwtResetTokenEncoder<'a>;
    type TokenNotifier = EmailResetTokenNotifier<'a>;

    fn extract(
        self,
    ) -> (
        Self::CheckNonceInfra,
        Self::ClockInfra,
        Self::PasswordInfra,
        Self::RequestDecoder,
        Self::DestinationRepository,
        Self::TokenGenerator,
        Self::TokenEncoder,
        Self::TokenNotifier,
        RequestResetTokenConfig,
    ) {
        (
            self.check_nonce_infra,
            self.clock_infra,
            self.password_infra,
            self.request_decoder,
            self.destination_repository,
            self.token_generator,
            self.token_encoder,
            self.token_notifier,
            self.config,
        )
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
        auth_ticket::_auth::kernel::init::test::{
            StaticAuthClockStruct, StaticCheckAuthNonceStruct,
        },
        password::_auth::kernel::init::test::StaticAuthUserPasswordStruct,
    };

    use super::super::infra::{RequestResetTokenConfig, RequestResetTokenInfra};

    pub struct StaticRequestResetTokenStruct<'a> {
        pub check_nonce_infra: StaticCheckAuthNonceStruct<'a>,
        pub clock_infra: StaticAuthClockStruct,
        pub password_infra: StaticAuthUserPasswordStruct<'a>,
        pub request_decoder: StaticRequestResetTokenRequestDecoder,
        pub destination_repository: MemoryResetTokenDestinationRepository<'a>,
        pub token_generator: StaticResetTokenGenerator,
        pub token_encoder: StaticResetTokenEncoder,
        pub token_notifier: StaticResetTokenNotifier,
        pub config: RequestResetTokenConfig,
    }

    impl<'a> RequestResetTokenInfra for StaticRequestResetTokenStruct<'a> {
        type CheckNonceInfra = StaticCheckAuthNonceStruct<'a>;
        type ClockInfra = StaticAuthClockStruct;
        type PasswordInfra = StaticAuthUserPasswordStruct<'a>;
        type RequestDecoder = StaticRequestResetTokenRequestDecoder;
        type DestinationRepository = MemoryResetTokenDestinationRepository<'a>;
        type TokenGenerator = StaticResetTokenGenerator;
        type TokenEncoder = StaticResetTokenEncoder;
        type TokenNotifier = StaticResetTokenNotifier;

        fn extract(
            self,
        ) -> (
            Self::CheckNonceInfra,
            Self::ClockInfra,
            Self::PasswordInfra,
            Self::RequestDecoder,
            Self::DestinationRepository,
            Self::TokenGenerator,
            Self::TokenEncoder,
            Self::TokenNotifier,
            RequestResetTokenConfig,
        ) {
            (
                self.check_nonce_infra,
                self.clock_infra,
                self.password_infra,
                self.request_decoder,
                self.destination_repository,
                self.token_generator,
                self.token_encoder,
                self.token_notifier,
                self.config,
            )
        }
    }
}
