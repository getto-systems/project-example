pub mod request_decoder;
pub mod search_repository;

use tonic::metadata::MetadataMap;

use crate::auth::user::account::remote::y_protobuf::service::SearchAuthUserAccountRequestPb;

use crate::x_outside_feature::remote::auth::feature::AuthAppFeature;

use crate::auth::{
    ticket::remote::validate::init::ApiValidateAuthTokenStruct,
    user::account::remote::search::init::{
        request_decoder::PbSearchAuthUserAccountRequestDecoder,
        search_repository::MysqlSearchAuthUserAccountRepository,
    },
};

use super::action::{SearchAuthUserAccountAction, SearchAuthUserAccountMaterial};

pub struct SearchAuthUserAccountFeature<'a> {
    validate: ApiValidateAuthTokenStruct<'a>,

    search_repository: MysqlSearchAuthUserAccountRepository<'a>,
}

impl<'a> SearchAuthUserAccountFeature<'a> {
    pub fn action(
        feature: &'a AuthAppFeature,
        metadata: &'a MetadataMap,
        request: SearchAuthUserAccountRequestPb,
    ) -> SearchAuthUserAccountAction<PbSearchAuthUserAccountRequestDecoder, Self> {
        SearchAuthUserAccountAction::with_material(
            PbSearchAuthUserAccountRequestDecoder::new(request),
            Self {
                validate: ApiValidateAuthTokenStruct::new(&feature.auth, metadata),

                search_repository: MysqlSearchAuthUserAccountRepository::new(
                    &feature.auth.store.mysql,
                ),
            },
        )
    }
}

impl<'a> SearchAuthUserAccountMaterial for SearchAuthUserAccountFeature<'a> {
    type Validate = ApiValidateAuthTokenStruct<'a>;

    type SearchRepository = MysqlSearchAuthUserAccountRepository<'a>;

    fn validate(&self) -> &Self::Validate {
        &self.validate
    }

    fn search_repository(&self) -> &Self::SearchRepository {
        &self.search_repository
    }
}
