pub mod request_decoder;

use tonic::metadata::MetadataMap;

use crate::auth::user::login_id::change::y_protobuf::service::OverrideLoginIdRequestPb;

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::auth::{
    ticket::validate::init::ApiValidateAuthTokenStruct,
    user::{
        kernel::init::user_repository::dynamodb::DynamoDbAuthUserRepository,
        login_id::change::init::request_decoder::PbOverrideLoginIdRequestDecoder,
    },
};

use super::action::{OverrideLoginIdAction, OverrideLoginIdMaterial};

pub struct OverrideLoginIdFeature<'a> {
    validate: ApiValidateAuthTokenStruct<'a>,
    user_repository: DynamoDbAuthUserRepository<'a>,
}

impl<'a> OverrideLoginIdFeature<'a> {
    pub fn action(
        feature: &'a AuthAppFeature,
        metadata: &'a MetadataMap,
        request: OverrideLoginIdRequestPb,
    ) -> OverrideLoginIdAction<PbOverrideLoginIdRequestDecoder, Self> {
        OverrideLoginIdAction::with_material(
            PbOverrideLoginIdRequestDecoder::new(request),
            Self {
                validate: ApiValidateAuthTokenStruct::new(feature, metadata),
                user_repository: DynamoDbAuthUserRepository::new(&feature.store),
            },
        )
    }
}

impl<'a> OverrideLoginIdMaterial for OverrideLoginIdFeature<'a> {
    type Validate = ApiValidateAuthTokenStruct<'a>;

    type LoginIdRepository = DynamoDbAuthUserRepository<'a>;

    fn validate(&self) -> &Self::Validate {
        &self.validate
    }
    fn login_id_repository(&self) -> &Self::LoginIdRepository {
        &self.user_repository
    }
}
