pub mod request_decoder;
pub mod token_encoder;
pub mod token_generator;
pub mod token_notifier;

use tonic::metadata::MetadataMap;

use crate::auth::user::password::reset::request_token::y_protobuf::service::RequestResetTokenRequestPb;

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::auth::{
    ticket::{kernel::init::clock::ChronoAuthClock, validate::init::ValidateAuthNonceStruct},
    user::{
        kernel::init::user_repository::dynamodb::DynamoDbAuthUserRepository,
        password::reset::request_token::init::{
            request_decoder::PbRequestResetTokenRequestDecoder,
            token_encoder::JwtResetTokenEncoder, token_generator::UuidResetTokenGenerator,
            token_notifier::EmailResetTokenNotifier,
        },
    },
};

use super::action::{RequestResetTokenAction, RequestResetTokenMaterial};

use crate::auth::user::password::reset::request_token::infra::RequestResetTokenConfig;

pub struct RequestResetTokenStruct<'a> {
    validate_nonce: ValidateAuthNonceStruct<'a>,

    clock: ChronoAuthClock,
    reset_token_repository: DynamoDbAuthUserRepository<'a>,
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
                validate_nonce: ValidateAuthNonceStruct::new(feature, metadata),

                clock: ChronoAuthClock::new(),
                reset_token_repository: DynamoDbAuthUserRepository::new(&feature.store),
                token_generator: UuidResetTokenGenerator,
                token_encoder: JwtResetTokenEncoder::new(&feature.reset_token_key),
                token_notifier: EmailResetTokenNotifier::new(&feature.email),
                config: RequestResetTokenConfig {
                    token_expires: feature.config.reset_token_expires,
                },
            },
        )
    }
}

impl<'a> RequestResetTokenMaterial for RequestResetTokenStruct<'a> {
    type ValidateNonce = ValidateAuthNonceStruct<'a>;

    type Clock = ChronoAuthClock;
    type ResetTokenRepository = DynamoDbAuthUserRepository<'a>;
    type TokenGenerator = UuidResetTokenGenerator;
    type TokenEncoder = JwtResetTokenEncoder<'a>;
    type TokenNotifier = EmailResetTokenNotifier<'a>;

    fn validate_nonce(&self) -> &Self::ValidateNonce {
        &self.validate_nonce
    }

    fn clock(&self) -> &Self::Clock {
        &self.clock
    }
    fn reset_token_repository(&self) -> &Self::ResetTokenRepository {
        &self.reset_token_repository
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
