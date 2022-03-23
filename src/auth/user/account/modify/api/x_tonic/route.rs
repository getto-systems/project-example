use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::auth::user::login_id::change::y_protobuf::service::{
    override_login_id_pb_server::OverrideLoginIdPb, OverrideLoginIdRequestPb,
    OverrideLoginIdResponsePb,
};

use crate::x_outside_feature::auth::{
    feature::{extract_request, TonicRequest},
    logger::app_logger,
};

use crate::x_content::metadata::metadata_request_id;

use crate::auth::user::login_id::change::init::OverrideLoginIdFeature;

use crate::z_lib::{logger::infra::Logger, response::tonic::ServiceResponder};

pub struct ServiceOverrideLoginId;

impl ServiceOverrideLoginId {
    pub const fn name() -> &'static str {
        "auth.user.loginId.override"
    }
}

#[async_trait::async_trait]
impl OverrideLoginIdPb for ServiceOverrideLoginId {
    async fn override_login_id(
        &self,
        request: Request<OverrideLoginIdRequestPb>,
    ) -> Result<Response<OverrideLoginIdResponsePb>, Status> {
        let TonicRequest {
            feature,
            metadata,
            request,
        } = extract_request(request);
        let request_id = metadata_request_id(&metadata);

        let logger = app_logger(Self::name(), request_id.into());
        let mut action = OverrideLoginIdFeature::action(&feature, &metadata, request);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite().await).respond_to()
    }
}
