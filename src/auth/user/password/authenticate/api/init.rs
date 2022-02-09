pub mod request_decoder;

use tonic::metadata::MetadataMap;

use crate::auth::user::password::authenticate::y_protobuf::service::AuthenticatePasswordRequestPb;

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::auth::{
    ticket::{
        encode::init::EncodeAuthTicketStruct, issue::init::IssueAuthTicketStruct,
        validate::init::ValidateAuthNonceStruct,
    },
    user::{
        kernel::init::user_repository::dynamodb::DynamoDbAuthUserRepository,
        password::{
            authenticate::init::request_decoder::PbAuthenticatePasswordRequestDecoder,
            kernel::init::password_matcher::Argon2PasswordMatcher,
        },
    },
};

use super::action::{AuthenticatePasswordAction, AuthenticatePasswordMaterial};

pub struct AuthenticatePasswordStruct<'a> {
    validate_nonce: ValidateAuthNonceStruct<'a>,
    issue: IssueAuthTicketStruct<'a>,
    encode: EncodeAuthTicketStruct<'a>,

    user_repository: DynamoDbAuthUserRepository<'a>,
}

impl<'a> AuthenticatePasswordStruct<'a> {
    pub fn action(
        feature: &'a AuthAppFeature,
        metadata: &'a MetadataMap,
        request: AuthenticatePasswordRequestPb,
    ) -> AuthenticatePasswordAction<PbAuthenticatePasswordRequestDecoder, Self> {
        AuthenticatePasswordAction::with_material(
            PbAuthenticatePasswordRequestDecoder::new(request),
            Self {
                validate_nonce: ValidateAuthNonceStruct::new(&feature.auth, metadata),
                issue: IssueAuthTicketStruct::new(&feature.auth),
                encode: EncodeAuthTicketStruct::new(&feature.auth),

                user_repository: DynamoDbAuthUserRepository::new(&feature.auth.store),
            },
        )
    }
}

impl<'a> AuthenticatePasswordMaterial for AuthenticatePasswordStruct<'a> {
    type ValidateNonce = ValidateAuthNonceStruct<'a>;
    type Issue = IssueAuthTicketStruct<'a>;
    type Encode = EncodeAuthTicketStruct<'a>;

    type UserRepository = DynamoDbAuthUserRepository<'a>;
    type PasswordRepository = DynamoDbAuthUserRepository<'a>;
    type PasswordMatcher = Argon2PasswordMatcher;

    fn validate_nonce(&self) -> &Self::ValidateNonce {
        &self.validate_nonce
    }
    fn issue(&self) -> &Self::Issue {
        &self.issue
    }
    fn encode(&self) -> &Self::Encode {
        &self.encode
    }

    fn user_repository(&self) -> &Self::UserRepository {
        &self.user_repository
    }
    fn password_repository(&self) -> &Self::PasswordRepository {
        &self.user_repository
    }
}
