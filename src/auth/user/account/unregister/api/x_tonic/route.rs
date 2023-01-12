use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::auth::user::account::unregister::y_protobuf::service::{
    unregister_auth_user_account_pb_server::UnregisterAuthUserAccountPb,
    UnregisterAuthUserAccountRequestPb, UnregisterAuthUserAccountResponsePb,
};

use crate::x_outside_feature::auth::{feature::AuthTonicRequest, logger::AuthLogger};

use crate::auth::user::account::unregister::init::ActiveUnregisterAuthUserAccountMaterial;

use crate::common::api::{logger::infra::Logger, response::tonic::ServiceResponder};

pub struct ServiceUnregisterUser;

#[async_trait::async_trait]
impl UnregisterAuthUserAccountPb for ServiceUnregisterUser {
    async fn unregister_user(
        &self,
        request: Request<UnregisterAuthUserAccountRequestPb>,
    ) -> Result<Response<UnregisterAuthUserAccountResponsePb>, Status> {
        let AuthTonicRequest {
            feature,
            metadata,
            request,
            request_id,
        } = AuthTonicRequest::from_request(request);

        let mut action =
            ActiveUnregisterAuthUserAccountMaterial::action(&feature, request_id.clone());
        let logger = AuthLogger::default(&feature, action.info.name(), request_id);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite(&metadata, request).await).respond_to()
    }
}
