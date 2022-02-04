use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::auth::user::password::remote::y_protobuf::service::{
    change_password_pb_server::ChangePasswordPb, ChangePasswordRequestPb, ChangePasswordResponsePb,
};

use crate::z_lib::remote::{logger::Logger, response::tonic::RespondTo};

use crate::x_outside_feature::remote::{
    auth::{
        feature::{extract_request, TonicRequest},
        logger::app_logger,
    },
    common::metadata::metadata_request_id,
};

use crate::auth::user::password::change::remote::init::ChangePasswordFeature;

pub struct ServiceChange;

impl ServiceChange {
    pub const fn name() -> &'static str {
        "auth.user.password.change"
    }
}

#[async_trait::async_trait]
impl ChangePasswordPb for ServiceChange {
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

        let logger = app_logger(Self::name(), request_id.into());
        let mut action = ChangePasswordFeature::action(&feature, &metadata, request);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite().await).respond_to()
    }
}
