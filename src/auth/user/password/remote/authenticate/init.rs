pub mod request_decoder;

use tonic::metadata::MetadataMap;

use crate::auth::user::password::remote::y_protobuf::service::AuthenticatePasswordRequestPb;

use crate::x_outside_feature::remote::auth::feature::AuthAppFeature;

use crate::auth::{
    ticket::remote::{
        check_nonce::init::CheckAuthNonceStruct, encode::init::EncodeAuthTicketStruct,
        issue::init::IssueAuthTicketStruct,
    },
    user::{
        password::remote::{
            authenticate::init::request_decoder::PbAuthenticatePasswordRequestDecoder,
            kernel::init::{
                password_matcher::Argon2PasswordMatcher,
                password_repository::MysqlAuthUserPasswordRepository,
            },
        },
        remote::kernel::init::user_repository::MysqlAuthUserRepository,
    },
};

use super::action::{AuthenticatePasswordAction, AuthenticatePasswordMaterial};

pub struct AuthenticatePasswordStruct<'a> {
    check_nonce: CheckAuthNonceStruct<'a>,
    issue: IssueAuthTicketStruct<'a>,
    encode: EncodeAuthTicketStruct<'a>,

    user_repository: MysqlAuthUserRepository<'a>,
    password_repository: MysqlAuthUserPasswordRepository<'a>,
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
                check_nonce: CheckAuthNonceStruct::new(&feature.auth, metadata),
                issue: IssueAuthTicketStruct::new(&feature.auth),
                encode: EncodeAuthTicketStruct::new(&feature.auth),

                user_repository: MysqlAuthUserRepository::new(&feature.auth.store.mysql),
                password_repository: MysqlAuthUserPasswordRepository::new(
                    &feature.auth.store.mysql,
                ),
            },
        )
    }
}

impl<'a> AuthenticatePasswordMaterial for AuthenticatePasswordStruct<'a> {
    type CheckNonce = CheckAuthNonceStruct<'a>;
    type Issue = IssueAuthTicketStruct<'a>;
    type Encode = EncodeAuthTicketStruct<'a>;

    type UserRepository = MysqlAuthUserRepository<'a>;
    type PasswordRepository = MysqlAuthUserPasswordRepository<'a>;
    type PasswordMatcher = Argon2PasswordMatcher;

    fn check_nonce(&self) -> &Self::CheckNonce {
        &self.check_nonce
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
        &self.password_repository
    }
}
