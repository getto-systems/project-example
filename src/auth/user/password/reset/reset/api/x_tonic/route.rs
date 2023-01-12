use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::auth::user::password::reset::reset::y_protobuf::service::{
    reset_password_pb_server::ResetPasswordPb, ResetPasswordRequestPb, ResetPasswordResponsePb,
};

use crate::x_outside_feature::auth::{feature::AuthTonicRequest, logger::AuthLogger};

use crate::auth::user::password::reset::reset::init::ActiveResetPasswordMaterial;

use crate::common::api::{logger::infra::Logger, response::tonic::ServiceResponder};

pub struct ServiceReset;

#[async_trait::async_trait]
impl ResetPasswordPb for ServiceReset {
    async fn reset(
        &self,
        request: Request<ResetPasswordRequestPb>,
    ) -> Result<Response<ResetPasswordResponsePb>, Status> {
        let AuthTonicRequest {
            feature,
            request,
            request_id,
            ..
        } = AuthTonicRequest::from_request(request);

        let mut action = ActiveResetPasswordMaterial::action(&feature);
        let logger = AuthLogger::default(&feature, action.info.name(), request_id);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite(request).await).respond_to()
    }
}
