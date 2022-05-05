use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::auth::user::login_id::change::y_protobuf::service::{
    overwrite_login_id_pb_server::OverwriteLoginIdPb, OverwriteLoginIdRequestPb,
    OverwriteLoginIdResponsePb,
};

use crate::x_outside_feature::auth::{
    feature::{extract_request, TonicRequest},
    logger::app_logger,
};

use crate::x_content::metadata::metadata_request_id;

use crate::auth::user::login_id::change::init::OverwriteLoginIdFeature;

use crate::z_lib::{logger::infra::Logger, response::tonic::ServiceResponder};

pub struct ServiceOverwriteLoginId;

impl ServiceOverwriteLoginId {
    pub const fn name() -> &'static str {
        "auth.user.loginId.overwrite"
    }
}

#[async_trait::async_trait]
impl OverwriteLoginIdPb for ServiceOverwriteLoginId {
    async fn overwrite_login_id(
        &self,
        request: Request<OverwriteLoginIdRequestPb>,
    ) -> Result<Response<OverwriteLoginIdResponsePb>, Status> {
        let TonicRequest {
            feature,
            metadata,
            request,
        } = extract_request(request);
        let request_id = metadata_request_id(&metadata);

        let logger = app_logger(Self::name(), request_id.into());
        let mut action = OverwriteLoginIdFeature::action(&feature, &metadata, request);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite().await).respond_to()
    }
}
