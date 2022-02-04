use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::z_lib::remote::{logger::Logger, response::tonic::RespondTo};

use crate::auth::user::password::reset::y_protobuf::service::{
    reset_password_pb_server::ResetPasswordPb, ResetPasswordRequestPb, ResetPasswordResponsePb,
};

use crate::x_outside_feature::remote::{
    auth::{
        feature::{extract_request, TonicRequest},
        logger::app_logger,
    },
    common::metadata::metadata_request_id,
};

use crate::auth::user::password::reset::reset::remote::init::ResetPasswordFeature;

pub struct ServiceReset;

impl ServiceReset {
    pub const fn name() -> &'static str {
        "auth.user.password.reset"
    }
}

#[async_trait::async_trait]
impl ResetPasswordPb for ServiceReset {
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

        let logger = app_logger(Self::name(), request_id.into());
        let mut action = ResetPasswordFeature::action(&feature, &metadata, request);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite().await).respond_to()
    }
}
