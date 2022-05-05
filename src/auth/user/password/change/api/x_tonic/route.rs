use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::auth::user::password::change::y_protobuf::service::{
    change_password_pb_server::ChangePasswordPb, overwrite_password_pb_server::OverwritePasswordPb,
    ChangePasswordRequestPb, ChangePasswordResponsePb, OverwritePasswordRequestPb,
    OverwritePasswordResponsePb,
};

use crate::x_outside_feature::auth::{
    feature::{extract_request, TonicRequest},
    logger::app_logger,
};

use crate::x_content::metadata::metadata_request_id;

use crate::auth::user::password::change::init::{ChangePasswordFeature, OverwritePasswordFeature};

use crate::z_lib::{logger::infra::Logger, response::tonic::ServiceResponder};

pub struct ServiceChangePassword;

impl ServiceChangePassword {
    pub const fn name() -> &'static str {
        "auth.user.password.change"
    }
}

#[async_trait::async_trait]
impl ChangePasswordPb for ServiceChangePassword {
    async fn change_password(
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

pub struct ServiceOverwritePassword;

impl ServiceOverwritePassword {
    pub const fn name() -> &'static str {
        "auth.user.password.overwrite"
    }
}

#[async_trait::async_trait]
impl OverwritePasswordPb for ServiceOverwritePassword {
    async fn overwrite_password(
        &self,
        request: Request<OverwritePasswordRequestPb>,
    ) -> Result<Response<OverwritePasswordResponsePb>, Status> {
        let TonicRequest {
            feature,
            metadata,
            request,
        } = extract_request(request);
        let request_id = metadata_request_id(&metadata);

        let logger = app_logger(Self::name(), request_id.into());
        let mut action = OverwritePasswordFeature::action(&feature, &metadata, request);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite().await).respond_to()
    }
}
