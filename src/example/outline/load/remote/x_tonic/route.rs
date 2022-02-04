use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::z_lib::remote::{logger::Logger, response::tonic::RespondTo};

use crate::x_outside_feature::remote::{
    common::metadata::metadata_request_id,
    example::{
        feature::{extract_request, TonicRequest},
        logger::app_logger,
    },
};

use crate::example::outline::remote::y_protobuf::service::{
    load_menu_badge_pb_server::LoadMenuBadgePb, LoadMenuBadgeRequestPb, LoadMenuBadgeResponsePb,
};

use crate::example::outline::load::remote::init::LoadOutlineMenuBadgeStruct;

pub struct ServiceGetMenuBadge;

#[async_trait::async_trait]
impl LoadMenuBadgePb for ServiceGetMenuBadge {
    async fn load_menu_badge(
        &self,
        request: Request<LoadMenuBadgeRequestPb>,
    ) -> Result<Response<LoadMenuBadgeResponsePb>, Status> {
        let TonicRequest {
            metadata, feature, ..
        } = extract_request(request);
        let request_id = metadata_request_id(&metadata);

        let logger = app_logger("example.outline.load_menu_badge", request_id.into());
        let mut action = LoadOutlineMenuBadgeStruct::action(&feature, &request_id, &metadata);
        action.subscribe(move |state| logger.log(state));

        flatten(action.ignite().await).respond_to()
    }
}
