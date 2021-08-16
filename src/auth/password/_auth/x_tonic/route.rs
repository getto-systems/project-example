use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::auth::password::reset::_auth::x_tonic::route::ResetServer;

use crate::z_details::_common::{logger::Logger, response::tonic::RespondTo};

use crate::auth::password::_common::y_protobuf::service::{
    authenticate_password_pb_server::{AuthenticatePasswordPb, AuthenticatePasswordPbServer},
    AuthenticatePasswordRequestPb, AuthenticatePasswordResponsePb,
};

use crate::x_outside_feature::_auth::{
    feature::{extract_request, TonicRequest},
    logger::app_logger,
};

use crate::auth::password::_auth::action_authenticate::init::AuthenticatePasswordFeature;

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
        let mut action = AuthenticatePasswordFeature::action(&data.auth, &metadata);
        action.subscribe(move |state| logger.log(state.log_level(), state));

        let request_decoder = AuthenticatePasswordFeature::request_decoder(request);
        flatten(action.ignite(request_decoder).await).respond_to()
    }
}
