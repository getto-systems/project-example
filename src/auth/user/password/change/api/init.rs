pub mod request_decoder;

use tonic::metadata::MetadataMap;

use crate::auth::user::password::change::y_protobuf::service::{
    ChangePasswordRequestPb, OverwritePasswordRequestPb,
};

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::auth::{
    ticket::validate::init::AuthenticateApiStruct,
    user::{
        kernel::init::user_repository::dynamodb::DynamoDbAuthUserRepository,
        password::{
            change::init::request_decoder::{
                PbChangePasswordRequestDecoder, PbOverwritePasswordRequestDecoder,
            },
            kernel::init::{
                password_hasher::Argon2PasswordHasher, password_matcher::Argon2PasswordMatcher,
            },
        },
    },
};

use super::action::{
    ChangePasswordAction, ChangePasswordMaterial, OverwritePasswordAction, OverwritePasswordMaterial,
};

pub struct ChangePasswordFeature<'a> {
    validate: AuthenticateApiStruct<'a>,
    user_repository: DynamoDbAuthUserRepository<'a>,
}

impl<'a> ChangePasswordFeature<'a> {
    pub fn action(
        feature: &'a AuthAppFeature,
        metadata: &'a MetadataMap,
        request: ChangePasswordRequestPb,
    ) -> ChangePasswordAction<PbChangePasswordRequestDecoder, Self> {
        ChangePasswordAction::with_material(
            PbChangePasswordRequestDecoder::new(request),
            Self {
                validate: AuthenticateApiStruct::new(feature, metadata),
                user_repository: DynamoDbAuthUserRepository::new(&feature.store),
            },
        )
    }
}

impl<'a> ChangePasswordMaterial for ChangePasswordFeature<'a> {
    type Authenticate = AuthenticateApiStruct<'a>;

    type PasswordRepository = DynamoDbAuthUserRepository<'a>;
    type PasswordMatcher = Argon2PasswordMatcher;
    type PasswordHasher = Argon2PasswordHasher;

    fn authenticate(&self) -> &Self::Authenticate {
        &self.validate
    }
    fn password_repository(&self) -> &Self::PasswordRepository {
        &self.user_repository
    }
}

pub struct OverwritePasswordFeature<'a> {
    validate: AuthenticateApiStruct<'a>,
    user_repository: DynamoDbAuthUserRepository<'a>,
}

impl<'a> OverwritePasswordFeature<'a> {
    pub fn action(
        feature: &'a AuthAppFeature,
        metadata: &'a MetadataMap,
        request: OverwritePasswordRequestPb,
    ) -> OverwritePasswordAction<PbOverwritePasswordRequestDecoder, Self> {
        OverwritePasswordAction::with_material(
            PbOverwritePasswordRequestDecoder::new(request),
            Self {
                validate: AuthenticateApiStruct::new(feature, metadata),
                user_repository: DynamoDbAuthUserRepository::new(&feature.store),
            },
        )
    }
}

impl<'a> OverwritePasswordMaterial for OverwritePasswordFeature<'a> {
    type Authenticate = AuthenticateApiStruct<'a>;

    type PasswordRepository = DynamoDbAuthUserRepository<'a>;
    type PasswordHasher = Argon2PasswordHasher;

    fn authenticate(&self) -> &Self::Authenticate {
        &self.validate
    }
    fn password_repository(&self) -> &Self::PasswordRepository {
        &self.user_repository
    }
}
