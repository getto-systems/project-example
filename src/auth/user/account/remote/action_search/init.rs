use tonic::metadata::MetadataMap;

use crate::auth::user::account::remote::y_protobuf::service::SearchAuthUserAccountRequestPb;

use crate::x_outside_feature::remote::auth::feature::AuthAppFeature;

use crate::auth::user::account::remote::search::init::{
    request_decoder::PbSearchAuthUserAccountRequestDecoder, SearchAuthUserAccountStruct,
};

use super::action::{SearchAuthUserAccountAction, SearchAuthUserAccountMaterial};

use crate::auth::user::account::remote::search::infra::SearchAuthUserAccountRequestDecoder;

pub struct SearchAuthUserAccountFeature<'a> {
    search: SearchAuthUserAccountStruct<'a>,
}

impl<'a> SearchAuthUserAccountFeature<'a> {
    pub fn action(
        feature: &'a AuthAppFeature,
        metadata: &'a MetadataMap,
    ) -> SearchAuthUserAccountAction<Self> {
        SearchAuthUserAccountAction::with_material(Self {
            search: SearchAuthUserAccountStruct::new(&feature.auth, metadata),
        })
    }
    pub fn request_decoder(
        request: SearchAuthUserAccountRequestPb,
    ) -> impl SearchAuthUserAccountRequestDecoder {
        PbSearchAuthUserAccountRequestDecoder::new(request)
    }
}

impl<'a> SearchAuthUserAccountMaterial for SearchAuthUserAccountFeature<'a> {
    type Search = SearchAuthUserAccountStruct<'a>;

    fn search(&self) -> &Self::Search {
        &self.search
    }
}
