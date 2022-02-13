use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::auth::ticket::logout::y_protobuf::service::{
    logout_pb_server::LogoutPb, LogoutRequestPb, LogoutResponsePb,
};

use crate::x_outside_feature::auth::{
    feature::{extract_request, TonicRequest},
    logger::app_logger,
};

use crate::x_content::metadata::metadata_request_id;

use crate::auth::ticket::logout::init::LogoutStruct;

use crate::z_lib::{logger::infra::Logger, response::tonic::ServiceResponder};

pub struct ServiceLogout;

impl ServiceLogout {
    pub const fn name() -> &'static str {
        "auth.ticket.logout"
    }
}

#[async_trait::async_trait]
impl LogoutPb for ServiceLogout {
    async fn logout(
        &self,
        request: Request<LogoutRequestPb>,
    ) -> Result<Response<LogoutResponsePb>, Status> {
        let TonicRequest {
            feature, metadata, ..
        } = extract_request(request);
        let request_id = metadata_request_id(&metadata);

        let logger = app_logger(Self::name(), request_id.into());
        let mut action = LogoutStruct::action(&feature, &metadata);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite().await).respond_to()
    }
}
