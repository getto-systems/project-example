use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::auth::user::password::reset::request_token::y_protobuf::service::{
    request_reset_token_pb_server::RequestResetTokenPb, RequestResetTokenRequestPb,
    RequestResetTokenResponsePb,
};

use crate::x_outside_feature::auth::{feature::AuthTonicRequest, logger::AuthLogger};

use crate::auth::user::password::reset::request_token::init::ActiveRequestResetTokenMaterial;

use crate::common::api::{logger::infra::Logger, response::tonic::ServiceResponder};

pub struct ServiceRequestToken;

#[async_trait::async_trait]
impl RequestResetTokenPb for ServiceRequestToken {
    async fn request_token(
        &self,
        request: Request<RequestResetTokenRequestPb>,
    ) -> Result<Response<RequestResetTokenResponsePb>, Status> {
        let AuthTonicRequest {
            feature,
            request,
            request_id,
            ..
        } = AuthTonicRequest::from_request(request);

        let mut action = ActiveRequestResetTokenMaterial::action(&feature);
        let logger = AuthLogger::default(&feature, action.info.name(), request_id);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite(request).await).respond_to()
    }
}
