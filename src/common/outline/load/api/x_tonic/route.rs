use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::common::outline::load::y_protobuf::service::{
    load_menu_badge_pb_server::LoadMenuBadgePb, LoadMenuBadgeRequestPb, LoadMenuBadgeResponsePb,
};

use crate::x_outside_feature::core::{
    feature::{extract_request, TonicRequest},
    logger::app_logger,
};

use crate::x_content::metadata::metadata_request_id;

use crate::common::outline::load::init::LoadOutlineMenuBadgeStruct;

use crate::z_lib::{logger::infra::Logger, response::tonic::ServiceResponder};

pub struct ServiceLoadMenuBadge;

impl ServiceLoadMenuBadge {
    pub const fn name() -> &'static str {
        "common.outline.load_menu_badge"
    }
}

#[async_trait::async_trait]
impl LoadMenuBadgePb for ServiceLoadMenuBadge {
    async fn load_menu_badge(
        &self,
        request: Request<LoadMenuBadgeRequestPb>,
    ) -> Result<Response<LoadMenuBadgeResponsePb>, Status> {
        let TonicRequest {
            metadata, feature, ..
        } = extract_request(request);
        let request_id = metadata_request_id(&metadata);

        let logger = app_logger(Self::name(), request_id.into());
        let mut action = LoadOutlineMenuBadgeStruct::action(&feature, &request_id, &metadata);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite().await).respond_to()
    }
}
