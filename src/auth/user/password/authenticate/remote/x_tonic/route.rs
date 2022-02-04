use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::auth::user::password::remote::y_protobuf::service::{
    authenticate_password_pb_server::AuthenticatePasswordPb, AuthenticatePasswordRequestPb,
    AuthenticatePasswordResponsePb,
};

use crate::z_lib::remote::{logger::Logger, response::tonic::RespondTo};

use crate::x_outside_feature::remote::{
    auth::{
        feature::{extract_request, TonicRequest},
        logger::app_logger,
    },
    common::metadata::metadata_request_id,
};

use crate::auth::user::password::authenticate::remote::init::AuthenticatePasswordStruct;

pub struct ServiceAuthenticate;

impl ServiceAuthenticate {
    pub const fn name() -> &'static str {
        "auth.user.password.authenticate"
    }
}

#[async_trait::async_trait]
impl AuthenticatePasswordPb for ServiceAuthenticate {
    async fn authenticate(
        &self,
        request: Request<AuthenticatePasswordRequestPb>,
    ) -> Result<Response<AuthenticatePasswordResponsePb>, Status> {
        let TonicRequest {
            feature,
            metadata,
            request,
        } = extract_request(request);
        let request_id = metadata_request_id(&metadata);

        let logger = app_logger(Self::name(), request_id.into());
        let mut action = AuthenticatePasswordStruct::action(&feature, &metadata, request);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite().await).respond_to()
    }
}
