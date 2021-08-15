use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::z_details::_common::{logger::Logger, response::tonic::RespondTo};

use crate::auth::password::reset::_common::y_protobuf::service::{
    request_reset_token_pb_server::{RequestResetTokenPb, RequestResetTokenPbServer},
    reset_password_pb_server::{ResetPasswordPb, ResetPasswordPbServer},
    RequestResetTokenRequestPb, RequestResetTokenResponsePb, ResetPasswordRequestPb,
    ResetPasswordResponsePb,
};

use crate::x_outside_feature::_auth::{
    feature::{extract_request, TonicRequest},
    logger::app_logger,
};

use crate::auth::password::reset::_auth::{
    action_request_token::init::RequestResetTokenFeature, action_reset::init::ResetPasswordFeature,
};

pub struct ResetServer;

impl ResetServer {
    pub fn request_token(&self) -> RequestResetTokenPbServer<RequestToken> {
        RequestResetTokenPbServer::new(RequestToken)
    }
    pub fn reset(&self) -> ResetPasswordPbServer<Reset> {
        ResetPasswordPbServer::new(Reset)
    }
}

pub struct RequestToken;

#[async_trait::async_trait]
impl RequestResetTokenPb for RequestToken {
    async fn request_token(
        &self,
        request: Request<RequestResetTokenRequestPb>,
    ) -> Result<Response<RequestResetTokenResponsePb>, Status> {
        let TonicRequest {
            data,
            metadata,
            request,
        } = extract_request(request);

        let logger = app_logger("auth.password.reset.request_token", &metadata);
        let mut action = RequestResetTokenFeature::action(&data.auth, &metadata);
        action.subscribe(move |state| logger.log(state.log_level(), state));

        let request_decoder = RequestResetTokenFeature::request_decoder(request);
        flatten(action.ignite(request_decoder).await).respond_to()
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
            data,
            metadata,
            request,
        } = extract_request(request);

        let logger = app_logger("auth.password.reset.reset", &metadata);
        let mut action = ResetPasswordFeature::action(&data.auth, &metadata);
        action.subscribe(move |state| logger.log(state.log_level(), state));

        let request_decoder = ResetPasswordFeature::request_decoder(request);
        flatten(action.ignite(request_decoder).await).respond_to()
    }
}
