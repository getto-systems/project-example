use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::auth::ticket::validate::y_protobuf::service::{
    authorize_pb_server::AuthorizePb, AuthorizeRequestPb, AuthorizeResponsePb,
};

use crate::x_outside_feature::auth::{
    feature::{extract_request, TonicRequest},
    logger::app_logger,
};

use crate::x_content::metadata::metadata_request_id;

use crate::auth::ticket::validate::init::AuthorizeStruct;

use crate::z_lib::{logger::infra::Logger, response::tonic::ServiceResponder};

pub struct ServiceAuthorize;

#[async_trait::async_trait]
impl AuthorizePb for ServiceAuthorize {
    async fn authorize(
        &self,
        request: Request<AuthorizeRequestPb>,
    ) -> Result<Response<AuthorizeResponsePb>, Status> {
        let TonicRequest {
            feature,
            metadata,
            request,
        } = extract_request(request);
        let request_id = metadata_request_id(&metadata);

        let logger = app_logger("auth.ticket.authorize", request_id.into());
        let mut action = AuthorizeStruct::action(&feature, &metadata, request);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite().await).respond_to()
    }
}
