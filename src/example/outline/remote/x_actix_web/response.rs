use crate::example::outline::remote::y_protobuf::api::GetMenuBadgeResultPb;

use crate::example::outline::remote::get_menu_badge::data::OutlineMenuBadge;

impl Into<GetMenuBadgeResultPb> for OutlineMenuBadge {
    fn into(self) -> GetMenuBadgeResultPb {
        GetMenuBadgeResultPb {
            index: self.index.extract(),
        }
    }
}
