mod proxy_service;
mod response_encoder;

use actix_web::HttpRequest;

use getto_application::infra::ActionStatePubSub;

use crate::x_outside_feature::_api::feature::ApiAppFeature;

use crate::auth::_api::init::{ApiAuthMetadata, JwtApiTokenDecoder};
use proxy_service::ProxyService;
use response_encoder::ResponseEncoder;

use crate::example::_api::proxy::{ExampleProxyEvent, ExampleProxyInfra};

use crate::example::outline::{
    _api::proxy_get_menu_badge::data::GetOutlineMenuBadgeProxyMessage,
    _common::data::OutlineMenuBadge,
};

pub struct GetOutlineMenuBadgeProxyStruct<'a> {
    pubsub: ActionStatePubSub<ExampleProxyEvent<GetOutlineMenuBadgeProxyMessage>>,
    auth_metadata: ApiAuthMetadata<'a>,
    token_decoder: JwtApiTokenDecoder<'a>,
    proxy_service: ProxyService<'a>,
    response_encoder: ResponseEncoder,
}

impl<'a> GetOutlineMenuBadgeProxyStruct<'a> {
    pub fn new(feature: &'a ApiAppFeature, request_id: &'a str, request: &'a HttpRequest) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            auth_metadata: ApiAuthMetadata::new(request),
            token_decoder: JwtApiTokenDecoder::new(&feature.auth.decoding_key),
            proxy_service: ProxyService::new(&feature.example.service, request_id),
            response_encoder: ResponseEncoder,
        }
    }

    pub fn subscribe(
        &mut self,
        handler: impl 'static + Fn(&ExampleProxyEvent<GetOutlineMenuBadgeProxyMessage>) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }
}

#[async_trait::async_trait]
impl<'a> ExampleProxyInfra<(), OutlineMenuBadge, GetOutlineMenuBadgeProxyMessage>
    for GetOutlineMenuBadgeProxyStruct<'a>
{
    type AuthMetadata = ApiAuthMetadata<'a>;
    type TokenDecoder = JwtApiTokenDecoder<'a>;
    type ProxyService = ProxyService<'a>;
    type ResponseEncoder = ResponseEncoder;

    fn auth_metadata(&self) -> &Self::AuthMetadata {
        &self.auth_metadata
    }
    fn token_decoder(&self) -> &Self::TokenDecoder {
        &self.token_decoder
    }
    fn proxy_service(&self) -> &Self::ProxyService {
        &self.proxy_service
    }
    fn response_encoder(&self) -> &Self::ResponseEncoder {
        &self.response_encoder
    }

    fn post(
        &self,
        state: ExampleProxyEvent<GetOutlineMenuBadgeProxyMessage>,
    ) -> ExampleProxyEvent<GetOutlineMenuBadgeProxyMessage> {
        self.pubsub.post(state)
    }
}
