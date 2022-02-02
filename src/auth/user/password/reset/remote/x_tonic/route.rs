use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::z_lib::remote::{logger::Logger, response::tonic::RespondTo};

use crate::auth::user::password::reset::remote::y_protobuf::service::{
    request_reset_token_pb_server::RequestResetTokenPbServer,
    reset_password_pb_server::{ResetPasswordPb, ResetPasswordPbServer},
    ResetPasswordRequestPb, ResetPasswordResponsePb,
};

use crate::x_outside_feature::remote::{
    auth::{
        feature::{extract_request, TonicRequest},
        logger::app_logger,
    },
    common::metadata::metadata_request_id,
};

use crate::auth::user::password::reset::request_token::remote::x_tonic::route::ServiceRequestToken;

use crate::auth::user::password::reset::remote::reset::init::ResetPasswordFeature;

pub struct ResetServer;

impl ResetServer {
    pub fn request_token(&self) -> RequestResetTokenPbServer<ServiceRequestToken> {
        RequestResetTokenPbServer::new(ServiceRequestToken)
    }
    pub fn reset(&self) -> ResetPasswordPbServer<Reset> {
        ResetPasswordPbServer::new(Reset)
    }
}

pub struct Reset;

#[async_trait::async_trait]
impl ResetPasswordPb for Reset {
    async fn reset(
        &self,
        request: Request<ResetPasswordRequestPb>,
    ) -> Result<Response<ResetPasswordResponsePb>, Status> {
        let TonicRequest {
            feature,
            metadata,
            request,
        } = extract_request(request);
        let request_id = metadata_request_id(&metadata);

        let logger = app_logger("auth.user.password.reset.reset", request_id.into());
        let mut action = ResetPasswordFeature::action(&feature, &metadata, request);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite().await).respond_to()
    }
}
