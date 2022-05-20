use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::avail::unexpected_error::notify::y_protobuf::service::{
    notify_pb_server::NotifyPb, NotifyRequestPb, NotifyResponsePb,
};

use crate::x_outside_feature::core::{
    feature::{extract_core_request, CoreTonicRequest},
    logger::app_logger,
};

use crate::x_content::metadata::metadata_request_id;

use crate::avail::unexpected_error::notify::init::NotifyUnexpectedErrorFeature;

use crate::z_lib::{logger::infra::Logger, response::tonic::ServiceResponder};

pub struct ServiceNotify;

impl ServiceNotify {
    pub const fn name() -> &'static str {
        "avail.unexpected_error.notify"
    }
}

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
        } = extract_core_request(request);
        let request_id = metadata_request_id(&metadata);
        let logger = app_logger(Self::name(), request_id.into());

        let mut action =
            NotifyUnexpectedErrorFeature::action(&feature, &request_id, &metadata, request);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite().await).respond_to()
    }
}
