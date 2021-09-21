mod proxy_service;
mod response_encoder;

use actix_web::HttpRequest;

use getto_application::infra::ActionStatePubSub;

use crate::auth::_api::init::ApiAuthMetadata;
use crate::example::_api::proxy::{ExampleProxyMaterial, ExampleProxyState};
use crate::example::outline::_api::proxy_get_menu_badge::data::GetOutlineMenuBadgeProxyMessage;
use crate::example::outline::_common::data::OutlineMenuBadge;
use crate::x_outside_feature::_api::feature::ApiAppFeature;
use proxy_service::ProxyService;
use response_encoder::ResponseEncoder;

pub struct GetOutlineMenuBadgeProxyFeature<'a> {
    pubsub: ActionStatePubSub<ExampleProxyState<GetOutlineMenuBadgeProxyMessage>>,
    auth_metadata: ApiAuthMetadata<'a>,
    proxy_service: ProxyService<'a>,
    response_encoder: ResponseEncoder,
}

impl<'a> GetOutlineMenuBadgeProxyFeature<'a> {
    pub fn new(
        feature: &'a ApiAppFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            auth_metadata: ApiAuthMetadata::new(&feature.auth.key, request),
            proxy_service: ProxyService::new(&feature.example.service, request_id),
            response_encoder: ResponseEncoder,
        }
    }

    pub fn subscribe(
        &mut self,
        handler: impl 'static + Fn(&ExampleProxyState<GetOutlineMenuBadgeProxyMessage>) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }
}

#[async_trait::async_trait]
impl<'a> ExampleProxyMaterial<(), OutlineMenuBadge, GetOutlineMenuBadgeProxyMessage>
    for GetOutlineMenuBadgeProxyFeature<'a>
{
    type AuthMetadata = ApiAuthMetadata<'a>;
    type ProxyService = ProxyService<'a>;
    type ResponseEncoder = ResponseEncoder;

    fn auth_metadata(&self) -> &Self::AuthMetadata {
        &self.auth_metadata
    }
    fn proxy_service(&self) -> &Self::ProxyService {
        &self.proxy_service
    }
    fn response_encoder(&self) -> &Self::ResponseEncoder {
        &self.response_encoder
    }

    fn post(
        &self,
        state: ExampleProxyState<GetOutlineMenuBadgeProxyMessage>,
    ) -> ExampleProxyState<GetOutlineMenuBadgeProxyMessage> {
        self.pubsub.post(state)
    }
}
