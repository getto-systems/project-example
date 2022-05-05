pub mod request_decoder;

use tonic::metadata::MetadataMap;

use crate::auth::user::login_id::change::y_protobuf::service::OverwriteLoginIdRequestPb;

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::auth::{
    ticket::validate::init::AuthenticateApiStruct,
    user::{
        kernel::init::user_repository::dynamodb::DynamoDbAuthUserRepository,
        login_id::change::init::request_decoder::PbOverwriteLoginIdRequestDecoder,
    },
};

use super::action::{OverwriteLoginIdAction, OverwriteLoginIdMaterial};

pub struct OverwriteLoginIdFeature<'a> {
    validate: AuthenticateApiStruct<'a>,
    user_repository: DynamoDbAuthUserRepository<'a>,
}

impl<'a> OverwriteLoginIdFeature<'a> {
    pub fn action(
        feature: &'a AuthAppFeature,
        metadata: &'a MetadataMap,
        request: OverwriteLoginIdRequestPb,
    ) -> OverwriteLoginIdAction<PbOverwriteLoginIdRequestDecoder, Self> {
        OverwriteLoginIdAction::with_material(
            PbOverwriteLoginIdRequestDecoder::new(request),
            Self {
                validate: AuthenticateApiStruct::new(feature, metadata),
                user_repository: DynamoDbAuthUserRepository::new(&feature.store),
            },
        )
    }
}

impl<'a> OverwriteLoginIdMaterial for OverwriteLoginIdFeature<'a> {
    type Authenticate = AuthenticateApiStruct<'a>;

    type LoginIdRepository = DynamoDbAuthUserRepository<'a>;

    fn authenticate(&self) -> &Self::Authenticate {
        &self.validate
    }
    fn login_id_repository(&self) -> &Self::LoginIdRepository {
        &self.user_repository
    }
}
