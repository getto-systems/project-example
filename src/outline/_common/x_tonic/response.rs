use crate::outline::_common::y_protobuf::service::GetMenuResponsePb;

use crate::outline::_common::data::{OutlineMenuBadge, OutlineMenuBadgeCount};

impl Into<OutlineMenuBadge> for GetMenuResponsePb {
    fn into(self) -> OutlineMenuBadge {
        OutlineMenuBadge {
            index: OutlineMenuBadgeCount::restore(self.index),
        }
    }
}
