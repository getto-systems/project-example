pub mod request_decoder;
pub mod reset_notifier;
pub mod token_decoder;

use tonic::metadata::MetadataMap;

use crate::auth::user::password::reset::reset::y_protobuf::service::ResetPasswordRequestPb;

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::auth::{
    ticket::{
        encode::init::EncodeAuthTicketStruct, issue::init::IssueAuthTicketStruct,
        kernel::init::clock::ChronoAuthClock, validate::init::ValidateAuthNonceStruct,
    },
    user::{
        kernel::init::user_repository::dynamodb::DynamoDbAuthUserRepository,
        password::{
            kernel::init::password_hasher::Argon2PasswordHasher,
            reset::reset::init::{
                request_decoder::PbResetPasswordRequestDecoder,
                reset_notifier::EmailResetPasswordNotifier, token_decoder::JwtResetTokenDecoder,
            },
        },
    },
};

use super::action::{ResetPasswordAction, ResetPasswordMaterial};

pub struct ResetPasswordFeature<'a> {
    validate_nonce: ValidateAuthNonceStruct<'a>,
    issue: IssueAuthTicketStruct<'a>,
    encode: EncodeAuthTicketStruct<'a>,

    clock: ChronoAuthClock,
    reset_password_repository: DynamoDbAuthUserRepository<'a>,
    token_decoder: JwtResetTokenDecoder<'a>,
    reset_notifier: EmailResetPasswordNotifier<'a>,
}

impl<'a> ResetPasswordFeature<'a> {
    pub fn action(
        feature: &'a AuthAppFeature,
        metadata: &'a MetadataMap,
        request: ResetPasswordRequestPb,
    ) -> ResetPasswordAction<PbResetPasswordRequestDecoder, Self> {
        ResetPasswordAction::with_material(
            PbResetPasswordRequestDecoder::new(request),
            Self {
                validate_nonce: ValidateAuthNonceStruct::new(feature, metadata),
                issue: IssueAuthTicketStruct::new(feature),
                encode: EncodeAuthTicketStruct::new(feature),

                clock: ChronoAuthClock::new(),
                reset_password_repository: DynamoDbAuthUserRepository::new(&feature.store),
                token_decoder: JwtResetTokenDecoder::new(&feature.reset_token_key),
                reset_notifier: EmailResetPasswordNotifier::new(&feature.email),
            },
        )
    }
}

impl<'a> ResetPasswordMaterial for ResetPasswordFeature<'a> {
    type ValidateNonce = ValidateAuthNonceStruct<'a>;
    type Issue = IssueAuthTicketStruct<'a>;
    type Encode = EncodeAuthTicketStruct<'a>;

    type Clock = ChronoAuthClock;
    type ResetPasswordRepository = DynamoDbAuthUserRepository<'a>;
    type PasswordHasher = Argon2PasswordHasher;
    type TokenDecoder = JwtResetTokenDecoder<'a>;
    type ResetNotifier = EmailResetPasswordNotifier<'a>;

    fn validate_nonce(&self) -> &Self::ValidateNonce {
        &self.validate_nonce
    }
    fn issue(&self) -> &Self::Issue {
        &self.issue
    }
    fn encode(&self) -> &Self::Encode {
        &self.encode
    }

    fn clock(&self) -> &Self::Clock {
        &self.clock
    }
    fn reset_password_repository(&self) -> &Self::ResetPasswordRepository {
        &self.reset_password_repository
    }
    fn token_decoder(&self) -> &Self::TokenDecoder {
        &self.token_decoder
    }
    fn reset_notifier(&self) -> &Self::ResetNotifier {
        &self.reset_notifier
    }
}
