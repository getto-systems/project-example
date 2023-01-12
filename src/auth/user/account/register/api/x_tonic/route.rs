use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::auth::user::account::register::y_protobuf::service::{
    register_auth_user_account_pb_server::RegisterAuthUserAccountPb,
    RegisterAuthUserAccountRequestPb, RegisterAuthUserAccountResponsePb,
};

use crate::x_outside_feature::auth::{feature::AuthTonicRequest, logger::AuthLogger};

use crate::auth::user::account::register::init::ActiveRegisterAuthUserAccountMaterial;

use crate::common::api::{logger::infra::Logger, response::tonic::ServiceResponder};

pub struct ServiceRegisterUser;

#[async_trait::async_trait]
impl RegisterAuthUserAccountPb for ServiceRegisterUser {
    async fn register_user(
        &self,
        request: Request<RegisterAuthUserAccountRequestPb>,
    ) -> Result<Response<RegisterAuthUserAccountResponsePb>, Status> {
        let AuthTonicRequest {
            feature,
            metadata,
            request,
            request_id,
        } = AuthTonicRequest::from_request(request);

        let mut action =
            ActiveRegisterAuthUserAccountMaterial::action(&feature, request_id.clone());
        let logger = AuthLogger::default(&feature, action.info.name(), request_id);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite(&metadata, request).await).respond_to()
    }
}
