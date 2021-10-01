use crate::example::outline::remote::y_protobuf::api::GetMenuBadgeApiResponsePb;

use crate::example::outline::remote::get_menu_badge::data::OutlineMenuBadge;

impl Into<GetMenuBadgeApiResponsePb> for OutlineMenuBadge {
    fn into(self) -> GetMenuBadgeApiResponsePb {
        GetMenuBadgeApiResponsePb {
            index: self.index.extract(),
        }
    }
}
