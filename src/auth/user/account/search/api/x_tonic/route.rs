use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::auth::user::account::search::y_protobuf::service::{
    search_auth_user_account_pb_server::SearchAuthUserAccountPb, SearchAuthUserAccountRequestPb,
    SearchAuthUserAccountResponsePb,
};

use crate::x_outside_feature::auth::{feature::AuthTonicRequest, logger::AuthLogger};

use crate::auth::user::account::search::init::ActiveSearchAuthUserAccountMaterial;

use crate::common::api::{logger::infra::Logger, response::tonic::ServiceResponder};

pub struct ServiceSearch;

#[async_trait::async_trait]
impl SearchAuthUserAccountPb for ServiceSearch {
    async fn search(
        &self,
        request: Request<SearchAuthUserAccountRequestPb>,
    ) -> Result<Response<SearchAuthUserAccountResponsePb>, Status> {
        let AuthTonicRequest {
            feature,
            metadata,
            request,
            request_id,
        } = AuthTonicRequest::from_request(request);

        let mut action = ActiveSearchAuthUserAccountMaterial::action(&feature, request_id.clone());
        let logger = AuthLogger::default(&feature, action.info.name(), request_id);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite(&metadata, request).await).respond_to()
    }
}
