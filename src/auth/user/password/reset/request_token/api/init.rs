pub mod request_decoder;
pub mod token_encoder;
pub mod token_generator;
pub mod token_notifier;

use tonic::metadata::MetadataMap;

use crate::auth::user::password::reset::request_token::y_protobuf::service::RequestResetTokenRequestPb;

use crate::x_outside_feature::api::auth::feature::AuthAppFeature;

use crate::auth::{
    ticket::{kernel::api::init::clock::ChronoAuthClock, validate::init::ValidateAuthNonceStruct},
    user::{
        kernel::init::user_repository::mysql::MysqlAuthUserRepository,
        password::reset::request_token::api::init::{
            request_decoder::PbRequestResetTokenRequestDecoder,
            token_encoder::JwtResetTokenEncoder, token_generator::UuidResetTokenGenerator,
            token_notifier::EmailResetTokenNotifier,
        },
    },
};

use super::action::{RequestResetTokenAction, RequestResetTokenMaterial};

use crate::auth::user::password::reset::request_token::api::infra::RequestResetTokenConfig;

pub struct RequestResetTokenStruct<'a> {
    validate_nonce: ValidateAuthNonceStruct<'a>,

    clock: ChronoAuthClock,
    user_repository: MysqlAuthUserRepository<'a>,
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
                user_repository: MysqlAuthUserRepository::new(&feature.auth.store.mysql),
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
    type PasswordRepository = MysqlAuthUserRepository<'a>;
    type DestinationRepository = MysqlAuthUserRepository<'a>;
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
        &self.user_repository
    }
    fn destination_repository(&self) -> &Self::DestinationRepository {
        &self.user_repository
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
