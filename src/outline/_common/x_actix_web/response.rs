use crate::outline::{_api::y_protobuf::api::GetMenuBadgeResultPb, _common::data::OutlineMenuBadge};

impl Into<GetMenuBadgeResultPb> for OutlineMenuBadge {
    fn into(self) -> GetMenuBadgeResultPb {
        GetMenuBadgeResultPb {
            index: self.index.extract(),
        }
    }
}
