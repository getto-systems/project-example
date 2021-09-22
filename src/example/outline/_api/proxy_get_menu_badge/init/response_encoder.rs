use crate::example::outline::_api::y_protobuf::api::GetMenuBadgeResultPb;

use crate::z_details::_api::message::helper::encode_protobuf_base64;

use crate::example::_api::proxy::ExampleProxyResponseEncoder;

use crate::{
    example::outline::{
        _api::proxy_get_menu_badge::data::GetOutlineMenuBadgeProxyMessage,
        _common::data::OutlineMenuBadge,
    },
    z_details::_api::message::data::MessageError,
};

pub struct ResponseEncoder;

impl<'a> ExampleProxyResponseEncoder<OutlineMenuBadge, GetOutlineMenuBadgeProxyMessage>
    for ResponseEncoder
{
    fn encode(
        &self,
        badge: OutlineMenuBadge,
    ) -> Result<GetOutlineMenuBadgeProxyMessage, MessageError> {
        let message: GetMenuBadgeResultPb = badge.into();
        Ok(GetOutlineMenuBadgeProxyMessage::Success(
            encode_protobuf_base64(message)?,
        ))
    }
}
