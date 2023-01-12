use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::auth::user::login_id::change::y_protobuf::service::{
    overwrite_login_id_pb_server::OverwriteLoginIdPb, OverwriteLoginIdRequestPb,
    OverwriteLoginIdResponsePb,
};

use crate::x_outside_feature::auth::{feature::AuthTonicRequest, logger::AuthLogger};

use crate::auth::user::login_id::change::init::ActiveOverwriteLoginIdMaterial;

use crate::common::api::{logger::infra::Logger, response::tonic::ServiceResponder};

pub struct ServiceOverwriteLoginId;

#[async_trait::async_trait]
impl OverwriteLoginIdPb for ServiceOverwriteLoginId {
    async fn overwrite_login_id(
        &self,
        request: Request<OverwriteLoginIdRequestPb>,
    ) -> Result<Response<OverwriteLoginIdResponsePb>, Status> {
        let AuthTonicRequest {
            feature,
            metadata,
            request,
            request_id,
        } = AuthTonicRequest::from_request(request);

        let mut action = ActiveOverwriteLoginIdMaterial::action(&feature, request_id.clone());
        let logger = AuthLogger::default(&feature, action.info.name(), request_id);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite(&metadata, request).await).respond_to()
    }
}
