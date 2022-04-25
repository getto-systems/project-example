pub mod request_decoder;

use tonic::metadata::MetadataMap;

use crate::auth::user::login_id::change::y_protobuf::service::OverrideLoginIdRequestPb;

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::auth::{
    ticket::validate::init::AuthenticateApiStruct,
    user::{
        kernel::init::user_repository::dynamodb::DynamoDbAuthUserRepository,
        login_id::change::init::request_decoder::PbOverrideLoginIdRequestDecoder,
    },
};

use super::action::{OverrideLoginIdAction, OverrideLoginIdMaterial};

pub struct OverrideLoginIdFeature<'a> {
    validate: AuthenticateApiStruct<'a>,
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
                validate: AuthenticateApiStruct::new(feature, metadata),
                user_repository: DynamoDbAuthUserRepository::new(&feature.store),
            },
        )
    }
}

impl<'a> OverrideLoginIdMaterial for OverrideLoginIdFeature<'a> {
    type Authenticate = AuthenticateApiStruct<'a>;

    type LoginIdRepository = DynamoDbAuthUserRepository<'a>;

    fn authenticate(&self) -> &Self::Authenticate {
        &self.validate
    }
    fn login_id_repository(&self) -> &Self::LoginIdRepository {
        &self.user_repository
    }
}
