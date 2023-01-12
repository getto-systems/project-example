use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::auth::user::password::change::y_protobuf::service::{
    change_password_pb_server::ChangePasswordPb, overwrite_password_pb_server::OverwritePasswordPb,
    ChangePasswordRequestPb, ChangePasswordResponsePb, OverwritePasswordRequestPb,
    OverwritePasswordResponsePb,
};

use crate::x_outside_feature::auth::{feature::AuthTonicRequest, logger::AuthLogger};

use crate::auth::user::password::change::init::{
    ActiveChangePasswordMaterial, ActiveOverwritePasswordMaterial,
};

use crate::common::api::{logger::infra::Logger, response::tonic::ServiceResponder};

pub struct ServiceChangePassword;

#[async_trait::async_trait]
impl ChangePasswordPb for ServiceChangePassword {
    async fn change_password(
        &self,
        request: Request<ChangePasswordRequestPb>,
    ) -> Result<Response<ChangePasswordResponsePb>, Status> {
        let AuthTonicRequest {
            feature,
            metadata,
            request,
            request_id,
        } = AuthTonicRequest::from_request(request);

        let mut action = ActiveChangePasswordMaterial::action(&feature, request_id.clone());
        let logger = AuthLogger::default(&feature, action.info.name(), request_id);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite(&metadata, request).await).respond_to()
    }
}

pub struct ServiceOverwritePassword;

#[async_trait::async_trait]
impl OverwritePasswordPb for ServiceOverwritePassword {
    async fn overwrite_password(
        &self,
        request: Request<OverwritePasswordRequestPb>,
    ) -> Result<Response<OverwritePasswordResponsePb>, Status> {
        let AuthTonicRequest {
            feature,
            metadata,
            request,
            request_id,
        } = AuthTonicRequest::from_request(request);

        let mut action = ActiveOverwritePasswordMaterial::action(&feature, request_id.clone());
        let logger = AuthLogger::default(&feature, action.info.name(), request_id);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite(&metadata, request).await).respond_to()
    }
}
