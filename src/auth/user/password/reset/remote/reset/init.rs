pub mod request_decoder;
pub mod reset_notifier;
pub mod token_decoder;

use tonic::metadata::MetadataMap;

use crate::auth::user::password::reset::remote::y_protobuf::service::ResetPasswordRequestPb;

use crate::x_outside_feature::remote::auth::feature::AuthAppFeature;

use crate::auth::{
    ticket::{
        encode::init::EncodeAuthTicketStruct,
        remote::{
            issue::init::IssueAuthTicketStruct, kernel::init::clock::ChronoAuthClock,
            validate::init::ValidateAuthNonceStruct,
        },
    },
    user::{
        password::{
            remote::kernel::init::{
                password_hasher::Argon2PasswordHasher,
                password_repository::MysqlAuthUserPasswordRepository,
            },
            reset::remote::reset::init::{
                request_decoder::PbResetPasswordRequestDecoder,
                reset_notifier::EmailResetPasswordNotifier, token_decoder::JwtResetTokenDecoder,
            },
        },
        remote::kernel::init::user_repository::MysqlAuthUserRepository,
    },
};

use super::action::{ResetPasswordAction, ResetPasswordMaterial};

pub struct ResetPasswordFeature<'a> {
    validate_nonce: ValidateAuthNonceStruct<'a>,
    issue: IssueAuthTicketStruct<'a>,
    encode: EncodeAuthTicketStruct<'a>,

    clock: ChronoAuthClock,
    user_repository: MysqlAuthUserRepository<'a>,
    password_repository: MysqlAuthUserPasswordRepository<'a>,
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
                validate_nonce: ValidateAuthNonceStruct::new(&feature.auth, metadata),
                issue: IssueAuthTicketStruct::new(&feature.auth),
                encode: EncodeAuthTicketStruct::new(&feature.auth),

                clock: ChronoAuthClock::new(),
                user_repository: MysqlAuthUserRepository::new(&feature.auth.store.mysql),
                password_repository: MysqlAuthUserPasswordRepository::new(
                    &feature.auth.store.mysql,
                ),
                token_decoder: JwtResetTokenDecoder::new(&feature.auth.reset_token_key),
                reset_notifier: EmailResetPasswordNotifier::new(&feature.auth.email),
            },
        )
    }
}

impl<'a> ResetPasswordMaterial for ResetPasswordFeature<'a> {
    type ValidateNonce = ValidateAuthNonceStruct<'a>;
    type Issue = IssueAuthTicketStruct<'a>;
    type Encode = EncodeAuthTicketStruct<'a>;

    type Clock = ChronoAuthClock;
    type UserRepository = MysqlAuthUserRepository<'a>;
    type PasswordRepository = MysqlAuthUserPasswordRepository<'a>;
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
    fn user_repository(&self) -> &Self::UserRepository {
        &self.user_repository
    }
    fn password_repository(&self) -> &Self::PasswordRepository {
        &self.password_repository
    }
    fn token_decoder(&self) -> &Self::TokenDecoder {
        &self.token_decoder
    }
    fn reset_notifier(&self) -> &Self::ResetNotifier {
        &self.reset_notifier
    }
}
