use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::auth::user::account::remote::y_protobuf::service::{
    search_user_account_pb_server::{SearchUserAccountPb, SearchUserAccountPbServer},
    SearchUserAccountRequestPb, SearchUserAccountResponsePb,
};

use crate::z_lib::remote::{logger::Logger, response::tonic::RespondTo};

use crate::x_outside_feature::remote::{
    auth::{
        feature::{extract_request, TonicRequest},
        logger::app_logger,
    },
    common::metadata::metadata_request_id,
};

use crate::auth::user::account::remote::action_search::init::SearchUserAccountFeature;

pub struct AccountServer;

impl AccountServer {
    pub fn search(&self) -> SearchUserAccountPbServer<Search> {
        SearchUserAccountPbServer::new(Search)
    }
}

pub struct Search;

#[async_trait::async_trait]
impl SearchUserAccountPb for Search {
    async fn search(
        &self,
        request: Request<SearchUserAccountRequestPb>,
    ) -> Result<Response<SearchUserAccountResponsePb>, Status> {
        let TonicRequest {
            data,
            metadata,
            request,
        } = extract_request(request);
        let request_id = metadata_request_id(&metadata);

        let logger = app_logger("auth.user.account.search", request_id.into());
        let mut action = SearchUserAccountFeature::action(&data, &metadata);
        action.subscribe(move |state| logger.log(state.log_level(), state));

        let request_decoder = SearchUserAccountFeature::request_decoder(request);
        flatten(action.ignite(request_decoder).await).respond_to()
    }
}
