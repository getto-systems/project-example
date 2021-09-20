use crate::auth::_common::infra::{AuthMetadata, AuthMetadataContent};

use crate::{
    example::{_api::service::data::ExampleServiceError, outline::_common::data::OutlineMenuBadge},
    z_details::_api::message::data::MessageError,
};

pub trait GetOutlineMenuBadgeInfra {
    type AuthMetadata: AuthMetadata;
    type GetMenuService: GetOutlineMenuBadgeService;
    type ResponseEncoder: GetOutlineMenuBadgeResponseEncoder;

    fn auth_metadata(&self) -> &Self::AuthMetadata;
    fn get_menu_service(&self) -> &Self::GetMenuService;
    fn response_encoder(&self) -> &Self::ResponseEncoder;
}

#[async_trait::async_trait]
pub trait GetOutlineMenuBadgeService {
    async fn get_menu(
        &self,
        metadata: AuthMetadataContent,
    ) -> Result<OutlineMenuBadge, ExampleServiceError>;
}

pub trait GetOutlineMenuBadgeResponseEncoder {
    fn encode(&self, badge: OutlineMenuBadge) -> Result<String, MessageError>;
}
