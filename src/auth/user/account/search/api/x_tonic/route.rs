use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::auth::user::account::search::y_protobuf::service::{
    search_auth_user_account_pb_server::SearchAuthUserAccountPb, SearchAuthUserAccountRequestPb,
    SearchAuthUserAccountResponsePb,
};

use crate::x_outside_feature::{
    auth::{
        feature::{extract_request, TonicRequest},
        logger::app_logger,
    },
    common::helper::metadata_request_id,
};

use crate::auth::user::account::search::init::SearchAuthUserAccountStruct;

use crate::z_lib::{logger::infra::Logger, response::tonic::ServiceResponder};

pub struct ServiceSearch;

impl ServiceSearch {
    pub const fn name() -> &'static str {
        "auth.user.account.search"
    }
}

#[async_trait::async_trait]
impl SearchAuthUserAccountPb for ServiceSearch {
    async fn search(
        &self,
        request: Request<SearchAuthUserAccountRequestPb>,
    ) -> Result<Response<SearchAuthUserAccountResponsePb>, Status> {
        let TonicRequest {
            feature,
            metadata,
            request,
        } = extract_request(request);
        let request_id = metadata_request_id(&metadata);

        let logger = app_logger(Self::name(), request_id.into());
        let mut action = SearchAuthUserAccountStruct::action(&feature, &metadata, request);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite().await).respond_to()
    }
}
