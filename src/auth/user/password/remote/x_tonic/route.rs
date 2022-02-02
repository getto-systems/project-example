use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::auth::user::password::remote::y_protobuf::service::{
    authenticate_password_pb_server::AuthenticatePasswordPbServer,
    change_password_pb_server::{ChangePasswordPb, ChangePasswordPbServer},
    ChangePasswordRequestPb, ChangePasswordResponsePb,
};

use crate::z_lib::remote::{logger::Logger, response::tonic::RespondTo};

use crate::x_outside_feature::remote::{
    auth::{
        feature::{extract_request, TonicRequest},
        logger::app_logger,
    },
    common::metadata::metadata_request_id,
};

use crate::auth::user::password::{
    authenticate::remote::x_tonic::route::ServiceAuthenticate,
    reset::remote::x_tonic::route::ResetServer,
};

use crate::auth::user::password::remote::change::init::ChangePasswordFeature;

pub struct PasswordServer {
    pub reset: ResetServer,
}

impl PasswordServer {
    pub const fn new() -> Self {
        Self { reset: ResetServer }
    }
    pub fn authenticate(&self) -> AuthenticatePasswordPbServer<ServiceAuthenticate> {
        AuthenticatePasswordPbServer::new(ServiceAuthenticate)
    }
    pub fn change(&self) -> ChangePasswordPbServer<Change> {
        ChangePasswordPbServer::new(Change)
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
            feature,
            metadata,
            request,
        } = extract_request(request);
        let request_id = metadata_request_id(&metadata);

        let logger = app_logger("auth.user.password.change", request_id.into());
        let mut action = ChangePasswordFeature::action(&feature, &metadata, request);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite().await).respond_to()
    }
}
