use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::avail::unexpected_error::notify::y_protobuf::service::{
    notify_pb_server::NotifyPb, NotifyRequestPb, NotifyResponsePb,
};

use crate::x_outside_feature::core::{feature::CoreTonicRequest, logger::CoreLogger};

use crate::avail::unexpected_error::notify::init::ActiveNotifyUnexpectedErrorMaterial;

use crate::common::api::{logger::infra::Logger, response::tonic::ServiceResponder};

pub struct ServiceNotify;

#[async_trait::async_trait]
impl NotifyPb for ServiceNotify {
    async fn notify(
        &self,
        request: Request<NotifyRequestPb>,
    ) -> Result<Response<NotifyResponsePb>, Status> {
        let CoreTonicRequest {
            metadata,
            feature,
            request,
            request_id,
        } = CoreTonicRequest::from_request(request);

        let mut action = ActiveNotifyUnexpectedErrorMaterial::action(&feature, request_id.clone());
        let logger = CoreLogger::default(&feature, action.info.name(), request_id);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite(&metadata, request).await).respond_to()
    }
}
