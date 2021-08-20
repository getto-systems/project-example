use crate::outline::_common::y_protobuf::service::GetMenuBadgeResponsePb;

use crate::outline::_common::data::{OutlineMenuBadge, OutlineMenuBadgeCount};

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
            index: self.index.extract()
        }
    }
}
