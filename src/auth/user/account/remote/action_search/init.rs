use tonic::metadata::MetadataMap;

use crate::auth::user::account::remote::y_protobuf::service::SearchUserAccountRequestPb;

use crate::x_outside_feature::remote::auth::feature::AuthAppFeature;

use crate::auth::user::account::remote::search::init::{
    request_decoder::PbSearchUserAccountRequestDecoder, SearchUserAccountStruct,
};

use super::action::{SearchUserAccountAction, SearchUserAccountMaterial};

use crate::auth::user::account::remote::search::infra::SearchUserAccountRequestDecoder;

pub struct SearchUserAccountFeature<'a> {
    search: SearchUserAccountStruct<'a>,
}

impl<'a> SearchUserAccountFeature<'a> {
    pub fn action(
        feature: &'a AuthAppFeature,
        metadata: &'a MetadataMap,
    ) -> SearchUserAccountAction<Self> {
        SearchUserAccountAction::with_material(Self {
            search: SearchUserAccountStruct::new(&feature.auth, metadata),
        })
    }
    pub fn request_decoder(
        request: SearchUserAccountRequestPb,
    ) -> impl SearchUserAccountRequestDecoder {
        PbSearchUserAccountRequestDecoder::new(request)
    }
}

impl<'a> SearchUserAccountMaterial for SearchUserAccountFeature<'a> {
    type Search = SearchUserAccountStruct<'a>;

    fn search(&self) -> &Self::Search {
        &self.search
    }
}
