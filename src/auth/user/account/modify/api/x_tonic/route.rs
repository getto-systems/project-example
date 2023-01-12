use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::auth::user::account::modify::y_protobuf::service::{
    modify_auth_user_account_pb_server::ModifyAuthUserAccountPb, ModifyAuthUserAccountRequestPb,
    ModifyAuthUserAccountResponsePb,
};

use crate::x_outside_feature::auth::{feature::AuthTonicRequest, logger::AuthLogger};

use crate::auth::user::account::modify::init::ActiveModifyAuthUserAccountMaterial;

use crate::common::api::{logger::infra::Logger, response::tonic::ServiceResponder};

pub struct ServiceModifyUser;

#[async_trait::async_trait]
impl ModifyAuthUserAccountPb for ServiceModifyUser {
    async fn modify_user(
        &self,
        request: Request<ModifyAuthUserAccountRequestPb>,
    ) -> Result<Response<ModifyAuthUserAccountResponsePb>, Status> {
        let AuthTonicRequest {
            feature,
            metadata,
            request,
            request_id,
        } = AuthTonicRequest::from_request(request);

        let mut action = ActiveModifyAuthUserAccountMaterial::action(&feature, request_id.clone());
        let logger = AuthLogger::default(&feature, action.info.name(), request_id);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite(&metadata, request).await).respond_to()
    }
}
