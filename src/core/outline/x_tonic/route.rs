use crate::core::outline::y_protobuf::service::load_menu_badge_pb_server::LoadMenuBadgePbServer;

use crate::core::outline::load::api::x_tonic::route::ServiceLoadMenuBadge;

pub struct OutlineServer;

impl OutlineServer {
    pub fn load_menu_badge(&self) -> LoadMenuBadgePbServer<ServiceLoadMenuBadge> {
        LoadMenuBadgePbServer::new(ServiceLoadMenuBadge)
    }
}
