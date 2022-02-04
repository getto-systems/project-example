use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::z_lib::api::{logger::Logger, response::tonic::RespondTo};

use crate::core::outline::y_protobuf::service::{
    load_menu_badge_pb_server::LoadMenuBadgePb, LoadMenuBadgeRequestPb, LoadMenuBadgeResponsePb,
};

use crate::x_outside_feature::api::{
    common::metadata::metadata_request_id,
    core::{
        feature::{extract_request, TonicRequest},
        logger::app_logger,
    },
};

use crate::core::outline::load::api::init::LoadOutlineMenuBadgeStruct;

pub struct ServiceLoadMenuBadge;

impl ServiceLoadMenuBadge {
    pub const fn name() -> &'static str {
        "example.outline.load_menu_badge"
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
