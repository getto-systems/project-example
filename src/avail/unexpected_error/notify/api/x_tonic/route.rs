use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::z_lib::{logger::Logger, response::tonic::ServiceResponder};

use crate::avail::unexpected_error::notify::y_protobuf::service::{
    notify_pb_server::NotifyPb, NotifyRequestPb, NotifyResponsePb,
};

use crate::x_outside_feature::{
    common::metadata::metadata_request_id,
    core::{
        feature::{extract_request, TonicRequest},
        logger::app_logger,
    },
};

use crate::avail::unexpected_error::notify::init::NotifyUnexpectedErrorFeature;

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
        let TonicRequest {
            metadata,
            feature,
            request,
        } = extract_request(request);
        let request_id = metadata_request_id(&metadata);
        let logger = app_logger(Self::name(), request_id.into());

        let mut action =
            NotifyUnexpectedErrorFeature::action(&feature, &request_id, &metadata, request);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite().await).respond_to()
    }
}
