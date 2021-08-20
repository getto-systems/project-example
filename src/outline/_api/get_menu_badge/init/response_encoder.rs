use crate::outline::_api::y_protobuf::api::GetMenuBadgeResultPb;

use crate::z_details::_api::message::helper::encode_protobuf_base64;

use super::super::infra::GetOutlineMenuBadgeResponseEncoder;

use crate::{
    outline::_common::data::OutlineMenuBadge, z_details::_api::message::data::MessageError,
};

pub struct ProstGetOutlineMenuBadgeResponseEncoder;

impl GetOutlineMenuBadgeResponseEncoder for ProstGetOutlineMenuBadgeResponseEncoder {
    fn encode(&self, badge: OutlineMenuBadge) -> Result<String, MessageError> {
        let message: GetMenuBadgeResultPb = badge.into();
        encode_protobuf_base64(message)
    }
}

#[cfg(test)]
pub mod test {
    use super::super::super::infra::GetOutlineMenuBadgeResponseEncoder;

    use crate::{
        outline::_common::data::OutlineMenuBadge, z_details::_api::message::data::MessageError,
    };

    pub struct StaticGetOutlineMenuBadgeResponseEncoder;

    impl GetOutlineMenuBadgeResponseEncoder for StaticGetOutlineMenuBadgeResponseEncoder {
        fn encode(&self, _badge: OutlineMenuBadge) -> Result<String, MessageError> {
            Ok("ENCODED".into())
        }
    }
}
