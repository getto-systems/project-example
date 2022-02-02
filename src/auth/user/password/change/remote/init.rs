pub mod request_decoder;

use tonic::metadata::MetadataMap;

use crate::auth::user::password::remote::y_protobuf::service::ChangePasswordRequestPb;

use crate::x_outside_feature::remote::auth::feature::AuthAppFeature;

use crate::auth::{
    ticket::validate::init::ApiValidateAuthTokenStruct,
    user::password::{
        change::remote::init::request_decoder::PbChangePasswordRequestDecoder,
        kernel::init::{
            password_hasher::Argon2PasswordHasher, password_matcher::Argon2PasswordMatcher,
            password_repository::MysqlAuthUserPasswordRepository,
        },
    },
};

use super::action::{ChangePasswordAction, ChangePasswordMaterial};

pub struct ChangePasswordFeature<'a> {
    validate: ApiValidateAuthTokenStruct<'a>,
    password_repository: MysqlAuthUserPasswordRepository<'a>,
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
                validate: ApiValidateAuthTokenStruct::new(&feature.auth, metadata),
                password_repository: MysqlAuthUserPasswordRepository::new(
                    &feature.auth.store.mysql,
                ),
            },
        )
    }
}

impl<'a> ChangePasswordMaterial for ChangePasswordFeature<'a> {
    type Validate = ApiValidateAuthTokenStruct<'a>;

    type PasswordRepository = MysqlAuthUserPasswordRepository<'a>;
    type PasswordMatcher = Argon2PasswordMatcher;
    type PasswordHasher = Argon2PasswordHasher;

    fn validate(&self) -> &Self::Validate {
        &self.validate
    }

    fn password_repository(&self) -> &Self::PasswordRepository {
        &self.password_repository
    }
}
