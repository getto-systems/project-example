pub mod request_decoder;

use tonic::metadata::MetadataMap;

use crate::auth::user::password::change::y_protobuf::service::{
    ChangePasswordRequestPb, OverridePasswordRequestPb,
};

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::auth::{
    ticket::validate::init::ApiValidateAuthTokenStruct,
    user::{
        kernel::init::user_repository::dynamodb::DynamoDbAuthUserRepository,
        password::{
            change::init::request_decoder::{
                PbChangePasswordRequestDecoder, PbOverridePasswordRequestDecoder,
            },
            kernel::init::{
                password_hasher::Argon2PasswordHasher, password_matcher::Argon2PasswordMatcher,
            },
        },
    },
};

use super::action::{
    ChangePasswordAction, ChangePasswordMaterial, OverridePasswordAction, OverridePasswordMaterial,
};

pub struct ChangePasswordFeature<'a> {
    validate: ApiValidateAuthTokenStruct<'a>,
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
                validate: ApiValidateAuthTokenStruct::new(feature, metadata),
                user_repository: DynamoDbAuthUserRepository::new(&feature.store),
            },
        )
    }
}

impl<'a> ChangePasswordMaterial for ChangePasswordFeature<'a> {
    type Validate = ApiValidateAuthTokenStruct<'a>;

    type PasswordRepository = DynamoDbAuthUserRepository<'a>;
    type PasswordMatcher = Argon2PasswordMatcher;
    type PasswordHasher = Argon2PasswordHasher;

    fn validate(&self) -> &Self::Validate {
        &self.validate
    }
    fn password_repository(&self) -> &Self::PasswordRepository {
        &self.user_repository
    }
}

pub struct OverridePasswordFeature<'a> {
    validate: ApiValidateAuthTokenStruct<'a>,
    user_repository: DynamoDbAuthUserRepository<'a>,
}

impl<'a> OverridePasswordFeature<'a> {
    pub fn action(
        feature: &'a AuthAppFeature,
        metadata: &'a MetadataMap,
        request: OverridePasswordRequestPb,
    ) -> OverridePasswordAction<PbOverridePasswordRequestDecoder, Self> {
        OverridePasswordAction::with_material(
            PbOverridePasswordRequestDecoder::new(request),
            Self {
                validate: ApiValidateAuthTokenStruct::new(feature, metadata),
                user_repository: DynamoDbAuthUserRepository::new(&feature.store),
            },
        )
    }
}

impl<'a> OverridePasswordMaterial for OverridePasswordFeature<'a> {
    type Validate = ApiValidateAuthTokenStruct<'a>;

    type PasswordRepository = DynamoDbAuthUserRepository<'a>;
    type PasswordHasher = Argon2PasswordHasher;

    fn validate(&self) -> &Self::Validate {
        &self.validate
    }
    fn password_repository(&self) -> &Self::PasswordRepository {
        &self.user_repository
    }
}
