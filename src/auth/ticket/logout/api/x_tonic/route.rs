use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::auth::ticket::logout::y_protobuf::service::{
    logout_pb_server::LogoutPb, LogoutRequestPb, LogoutResponsePb,
};

use crate::x_outside_feature::auth::{feature::AuthTonicRequest, logger::AuthLogger};

use crate::auth::ticket::logout::init::ActiveLogoutMaterial;

use crate::common::api::{logger::infra::Logger, response::tonic::ServiceResponder};

pub struct ServiceLogout;

#[async_trait::async_trait]
impl LogoutPb for ServiceLogout {
    async fn logout(
        &self,
        request: Request<LogoutRequestPb>,
    ) -> Result<Response<LogoutResponsePb>, Status> {
        let AuthTonicRequest {
            feature,
            metadata,
            request_id,
            ..
        } = AuthTonicRequest::from_request(request);

        let mut action = ActiveLogoutMaterial::action(&feature);
        let logger = AuthLogger::default(&feature, action.info.name(), request_id);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite(&metadata).await).respond_to()
    }
}
