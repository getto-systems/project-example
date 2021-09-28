use crate::example::outline::remote::y_protobuf::service::GetMenuBadgeResponsePb;

use crate::example::outline::remote::get_menu_badge::data::{OutlineMenuBadge, OutlineMenuBadgeCount};

impl Into<OutlineMenuBadge> for GetMenuBadgeResponsePb {
    fn into(self) -> OutlineMenuBadge {
        OutlineMenuBadge {
            index: OutlineMenuBadgeCount::restore(self.index),
        }
    }
}

impl Into<GetMenuBadgeResponsePb> for OutlineMenuBadge {
    fn into(self) -> GetMenuBadgeResponsePb {
        GetMenuBadgeResponsePb {
            index: self.index.extract(),
        }
    }
}
