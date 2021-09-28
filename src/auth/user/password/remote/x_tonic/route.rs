use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::auth::user::password::_common::y_protobuf::service::{
    authenticate_password_pb_server::{AuthenticatePasswordPb, AuthenticatePasswordPbServer},
    change_password_pb_server::{ChangePasswordPb, ChangePasswordPbServer},
    AuthenticatePasswordRequestPb, AuthenticatePasswordResponsePb, ChangePasswordRequestPb,
    ChangePasswordResponsePb,
};

use crate::z_details::_common::{logger::Logger, response::tonic::RespondTo};

use crate::x_outside_feature::remote::auth::{
    feature::{extract_request, TonicRequest},
    logger::app_logger,
};

use crate::auth::user::password::reset::remote::x_tonic::route::ResetServer;

use crate::auth::user::password::remote::{
    action_authenticate::init::AuthenticatePasswordFeature,
    action_change::init::ChangePasswordFeature,
};

pub struct PasswordServer {
    pub reset: ResetServer,
}

impl PasswordServer {
    pub const fn new() -> Self {
        Self { reset: ResetServer }
    }
    pub fn authenticate(&self) -> AuthenticatePasswordPbServer<Authenticate> {
        AuthenticatePasswordPbServer::new(Authenticate)
    }
    pub fn change(&self) -> ChangePasswordPbServer<Change> {
        ChangePasswordPbServer::new(Change)
    }
}

pub struct Authenticate;

#[async_trait::async_trait]
impl AuthenticatePasswordPb for Authenticate {
    async fn authenticate(
        &self,
        request: Request<AuthenticatePasswordRequestPb>,
    ) -> Result<Response<AuthenticatePasswordResponsePb>, Status> {
        let TonicRequest {
            data,
            metadata,
            request,
        } = extract_request(request);

        let logger = app_logger("auth.password.authenticate", &metadata);
        let mut action = AuthenticatePasswordFeature::action(&data, &metadata);
        action.subscribe(move |state| logger.log(state.log_level(), state));

        let request_decoder = AuthenticatePasswordFeature::request_decoder(request);
        flatten(action.ignite(request_decoder).await).respond_to()
    }
}

pub struct Change;

#[async_trait::async_trait]
impl ChangePasswordPb for Change {
    async fn change(
        &self,
        request: Request<ChangePasswordRequestPb>,
    ) -> Result<Response<ChangePasswordResponsePb>, Status> {
        let TonicRequest {
            data,
            metadata,
            request,
        } = extract_request(request);

        let logger = app_logger("auth.password.change", &metadata);
        let mut action = ChangePasswordFeature::action(&data, &metadata);
        action.subscribe(move |state| logger.log(state.log_level(), state));

        let request_decoder = ChangePasswordFeature::request_decoder(request);
        flatten(action.ignite(request_decoder).await).respond_to()
    }
}
