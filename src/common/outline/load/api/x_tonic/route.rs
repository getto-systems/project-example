use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::common::outline::load::y_protobuf::service::{
    load_menu_badge_pb_server::LoadMenuBadgePb, LoadMenuBadgeRequestPb, LoadMenuBadgeResponsePb,
};

use crate::x_outside_feature::core::{feature::CoreTonicRequest, logger::CoreLogger};

use crate::common::outline::load::init::ActiveLoadOutlineMenuBadgeMaterial;

use crate::common::api::{logger::infra::Logger, response::tonic::ServiceResponder};

pub struct ServiceLoadMenuBadge;

#[async_trait::async_trait]
impl LoadMenuBadgePb for ServiceLoadMenuBadge {
    async fn load_menu_badge(
        &self,
        request: Request<LoadMenuBadgeRequestPb>,
    ) -> Result<Response<LoadMenuBadgeResponsePb>, Status> {
        let CoreTonicRequest {
            feature,
            metadata,
            request_id,
            ..
        } = CoreTonicRequest::from_request(request);

        let mut action = ActiveLoadOutlineMenuBadgeMaterial::action(&feature, request_id.clone());
        let logger = CoreLogger::default(&feature, action.info.name(), request_id);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite(&metadata).await).respond_to()
    }
}
