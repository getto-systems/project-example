use crate::example::outline::remote::y_protobuf::api::GetMenuBadgeApiResponsePb;

use crate::z_lib::remote::message::helper::encode_protobuf_base64;

use crate::example::remote::proxy::ExampleProxyResponseEncoder;

use crate::{
    example::outline::remote::{
        get_menu_badge::data::OutlineMenuBadge,
        proxy_get_menu_badge::data::GetOutlineMenuBadgeProxyMessage,
    },
    z_lib::remote::message::data::MessageError,
};

pub struct ResponseEncoder;

impl<'a> ExampleProxyResponseEncoder<OutlineMenuBadge, GetOutlineMenuBadgeProxyMessage>
    for ResponseEncoder
{
    fn encode(
        &self,
        badge: OutlineMenuBadge,
    ) -> Result<GetOutlineMenuBadgeProxyMessage, MessageError> {
        let message: GetMenuBadgeApiResponsePb = badge.into();
        Ok(GetOutlineMenuBadgeProxyMessage::Success(
            encode_protobuf_base64(message)?,
        ))
    }
}