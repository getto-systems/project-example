pub mod destination_repository;
pub mod request_decoder;
pub mod token_encoder;
pub mod token_generator;
pub mod token_notifier;

use tonic::metadata::MetadataMap;

use crate::auth::{
    ticket::validate::init::ValidateAuthNonceStruct,
    user::password::reset::remote::y_protobuf::service::RequestResetTokenRequestPb,
};

use crate::x_outside_feature::remote::auth::feature::AuthAppFeature;

use crate::auth::{
    ticket::kernel::remote::init::clock::ChronoAuthClock,
    user::password::{
        kernel::init::password_repository::MysqlAuthUserPasswordRepository,
        reset::request_token::remote::init::{
            destination_repository::MysqlResetTokenDestinationRepository,
            request_decoder::PbRequestResetTokenRequestDecoder,
            token_encoder::JwtResetTokenEncoder, token_generator::UuidResetTokenGenerator,
            token_notifier::EmailResetTokenNotifier,
        },
    },
};

use super::action::{RequestResetTokenAction, RequestResetTokenMaterial};

use crate::auth::user::password::reset::request_token::remote::infra::RequestResetTokenConfig;

pub struct RequestResetTokenStruct<'a> {
    validate_nonce: ValidateAuthNonceStruct<'a>,

    clock: ChronoAuthClock,
    password_repository: MysqlAuthUserPasswordRepository<'a>,
    destination_repository: MysqlResetTokenDestinationRepository<'a>,
    token_generator: UuidResetTokenGenerator,
    token_encoder: JwtResetTokenEncoder<'a>,
    token_notifier: EmailResetTokenNotifier<'a>,
    config: RequestResetTokenConfig,
}

impl<'a> RequestResetTokenStruct<'a> {
    pub fn action(
        feature: &'a AuthAppFeature,
        metadata: &'a MetadataMap,
        request: RequestResetTokenRequestPb,
    ) -> RequestResetTokenAction<PbRequestResetTokenRequestDecoder, Self> {
        RequestResetTokenAction::with_material(
            PbRequestResetTokenRequestDecoder::new(request),
            Self {
                validate_nonce: ValidateAuthNonceStruct::new(&feature.auth, metadata),

                clock: ChronoAuthClock::new(),
                password_repository: MysqlAuthUserPasswordRepository::new(
                    &feature.auth.store.mysql,
                ),
                destination_repository: MysqlResetTokenDestinationRepository::new(
                    &feature.auth.store.mysql,
                ),
                token_generator: UuidResetTokenGenerator,
                token_encoder: JwtResetTokenEncoder::new(&feature.auth.reset_token_key),
                token_notifier: EmailResetTokenNotifier::new(&feature.auth.email),
                config: RequestResetTokenConfig {
                    token_expires: feature.auth.config.reset_token_expires,
                },
            },
        )
    }
}

impl<'a> RequestResetTokenMaterial for RequestResetTokenStruct<'a> {
    type ValidateNonce = ValidateAuthNonceStruct<'a>;

    type Clock = ChronoAuthClock;
    type PasswordRepository = MysqlAuthUserPasswordRepository<'a>;
    type DestinationRepository = MysqlResetTokenDestinationRepository<'a>;
    type TokenGenerator = UuidResetTokenGenerator;
    type TokenEncoder = JwtResetTokenEncoder<'a>;
    type TokenNotifier = EmailResetTokenNotifier<'a>;

    fn validate_nonce(&self) -> &Self::ValidateNonce {
        &self.validate_nonce
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
