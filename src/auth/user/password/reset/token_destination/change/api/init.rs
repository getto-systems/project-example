pub mod request_decoder;

use tonic::metadata::MetadataMap;

use crate::auth::user::account::modify::y_protobuf::service::ModifyAuthUserAccountRequestPb;

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::auth::{
    ticket::validate::init::ApiValidateAuthTokenStruct,
    user::{
        account::modify::init::request_decoder::PbModifyAuthUserAccountRequestDecoder,
        kernel::init::user_repository::dynamodb::DynamoDbAuthUserRepository,
    },
};

use super::action::{ModifyAuthUserAccountAction, ModifyAuthUserAccountMaterial};

pub struct ModifyAuthUserAccountFeature<'a> {
    validate: ApiValidateAuthTokenStruct<'a>,
    user_repository: DynamoDbAuthUserRepository<'a>,
}

impl<'a> ModifyAuthUserAccountFeature<'a> {
    pub fn action(
        feature: &'a AuthAppFeature,
        metadata: &'a MetadataMap,
        request: ModifyAuthUserAccountRequestPb,
    ) -> ModifyAuthUserAccountAction<PbModifyAuthUserAccountRequestDecoder, Self> {
        ModifyAuthUserAccountAction::with_material(
            PbModifyAuthUserAccountRequestDecoder::new(request),
            Self {
                validate: ApiValidateAuthTokenStruct::new(feature, metadata),
                user_repository: DynamoDbAuthUserRepository::new(&feature.store),
            },
        )
    }
}

impl<'a> ModifyAuthUserAccountMaterial for ModifyAuthUserAccountFeature<'a> {
    type Validate = ApiValidateAuthTokenStruct<'a>;

    type UserRepository = DynamoDbAuthUserRepository<'a>;

    fn validate(&self) -> &Self::Validate {
        &self.validate
    }
    fn user_repository(&self) -> &Self::UserRepository {
        &self.user_repository
    }
}
