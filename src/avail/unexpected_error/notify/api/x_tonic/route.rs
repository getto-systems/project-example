use std::sync::Arc;

use tonic::{Request, Response, Status};

use crate::avail::unexpected_error::notify::y_protobuf::service::{
    notify_pb_server::{NotifyPb, NotifyPbServer},
    NotifyRequestPb, NotifyResponsePb,
};

use crate::x_outside_feature::core::feature::CoreAppFeature;

use crate::common::api::{
    feature::AsInfra, logger::detail::StdoutJsonLogger, response::x_tonic::ServiceResponder,
};

use crate::{
    auth::action::AuthorizeProxyAction,
    avail::unexpected_error::notify::action::NotifyUnexpectedErrorAction,
};

use crate::avail::unexpected_error::notify::infra::NotifyUnexpectedErrorFieldsExtract;

use crate::{
    auth::data::AuthorizeProxyError, avail::unexpected_error::notify::data::NotifyUnexpectedError,
    common::api::request::data::RequestInfo,
};

#[derive(Default)]
pub struct ServiceNotify;

impl ServiceNotify {
    pub fn server(&self) -> NotifyPbServer<Self> {
        NotifyPbServer::new(Self)
    }
}

#[async_trait::async_trait]
impl NotifyPb for ServiceNotify {
    async fn notify(
        &self,
        request: Request<NotifyRequestPb>,
    ) -> Result<Response<NotifyResponsePb>, Status> {
        async {
            let feature = CoreAppFeature::from_request(&request);
            let info = RequestInfo::from_metadata(request.metadata());
            let logger = Arc::new(StdoutJsonLogger::with_request(info.clone()));

            let infra = AuthorizeProxyAction::live(feature.as_infra())
                .with_logger(logger.clone())
                .pick_authorized_infra(&feature, info, request.metadata())
                .await?;

            Ok::<_, AuthorizeProxyError>(
                NotifyUnexpectedErrorAction::live(infra)
                    .with_logger(logger.clone())
                    .notify(request.into_inner())
                    .await,
            )
        }
        .await
        .respond_to()
    }
}

impl NotifyUnexpectedErrorFieldsExtract for NotifyRequestPb {
    fn convert(self) -> NotifyUnexpectedError {
        NotifyUnexpectedError::new(self.err)
    }
}

impl ServiceResponder<NotifyResponsePb> for NotifyUnexpectedError {
    fn respond_to(self) -> Result<Response<NotifyResponsePb>, Status> {
        Ok(Response::new(NotifyResponsePb {}))
    }
}
