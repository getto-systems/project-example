use tonic::{Request, Response, Status};

use getto_application::helper::flatten;

use crate::z_details::_common::{logger::Logger, response::tonic::RespondTo};

use crate::x_outside_feature::remote::{
    common::metadata::metadata_request_id,
    example::{
        feature::{extract_request, TonicRequest},
        logger::app_logger,
    },
};

use crate::example::outline::_common::y_protobuf::service::{
    get_menu_badge_pb_server::{GetMenuBadgePb, GetMenuBadgePbServer},
    GetMenuBadgeRequestPb, GetMenuBadgeResponsePb,
};

use crate::example::outline::remote::action_get_menu_badge::init::GetOutlineMenuBadgeFeature;

pub struct OutlineServer;

impl OutlineServer {
    pub fn get_menu_badge(&self) -> GetMenuBadgePbServer<GetMenuBadge> {
        GetMenuBadgePbServer::new(GetMenuBadge)
    }
}

pub struct GetMenuBadge;

#[async_trait::async_trait]
impl GetMenuBadgePb for GetMenuBadge {
    async fn get_menu_badge(
        &self,
        request: Request<GetMenuBadgeRequestPb>,
    ) -> Result<Response<GetMenuBadgeResponsePb>, Status> {
        let TonicRequest { metadata, data, .. } = extract_request(request);
        let request_id = metadata_request_id(&metadata);

        let logger = app_logger("example.outline.get_menu_badge", request_id.into());
        let mut action = GetOutlineMenuBadgeFeature::action(&data, &request_id, &metadata);
        action.subscribe(move |state| logger.log(state.log_level(), state));

        flatten(action.ignite().await).respond_to()
    }
}
