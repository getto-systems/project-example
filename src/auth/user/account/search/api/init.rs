pub mod request_decoder;

use tonic::metadata::MetadataMap;

use crate::auth::user::account::search::y_protobuf::service::SearchAuthUserAccountRequestPb;

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::auth::{
    ticket::validate::init::AuthenticateApiStruct,
    user::{
        account::search::init::request_decoder::PbSearchAuthUserAccountRequestDecoder,
        kernel::init::user_repository::dynamodb::DynamoDbAuthUserRepository,
    },
};

use crate::auth::user::account::search::action::{
    SearchAuthUserAccountAction, SearchAuthUserAccountMaterial,
};

pub struct SearchAuthUserAccountStruct<'a> {
    validate: AuthenticateApiStruct<'a>,
    user_repository: DynamoDbAuthUserRepository<'a>,
}

impl<'a> SearchAuthUserAccountStruct<'a> {
    pub fn action(
        feature: &'a AuthAppFeature,
        metadata: &'a MetadataMap,
        request: SearchAuthUserAccountRequestPb,
    ) -> SearchAuthUserAccountAction<PbSearchAuthUserAccountRequestDecoder, Self> {
        SearchAuthUserAccountAction::with_material(
            PbSearchAuthUserAccountRequestDecoder::new(request),
            Self {
                validate: AuthenticateApiStruct::new(feature, metadata),
                user_repository: DynamoDbAuthUserRepository::new(&feature.store),
            },
        )
    }
}

impl<'a> SearchAuthUserAccountMaterial for SearchAuthUserAccountStruct<'a> {
    type Authenticate = AuthenticateApiStruct<'a>;
    type SearchRepository = DynamoDbAuthUserRepository<'a>;

    fn authenticate(&self) -> &Self::Authenticate {
        &self.validate
    }
    fn search_repository(&self) -> &Self::SearchRepository {
        &self.user_repository
    }
}
