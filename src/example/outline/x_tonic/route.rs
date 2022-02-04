use crate::example::outline::remote::y_protobuf::service::load_menu_badge_pb_server::LoadMenuBadgePbServer;

use crate::example::outline::load::remote::x_tonic::route::ServiceLoadMenuBadge;

pub struct OutlineServer;

impl OutlineServer {
    pub fn load_menu_badge(&self) -> LoadMenuBadgePbServer<ServiceLoadMenuBadge> {
        LoadMenuBadgePbServer::new(ServiceLoadMenuBadge)
    }
}
