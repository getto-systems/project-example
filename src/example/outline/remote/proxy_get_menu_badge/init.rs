mod proxy_service;
mod response_encoder;

use actix_web::HttpRequest;

use getto_application::infra::ActionStatePubSub;

use crate::x_outside_feature::_api::feature::ApiAppFeature;

use crate::auth::remote::init::ValidateApiMetadataStruct;
use proxy_service::ProxyService;
use response_encoder::ResponseEncoder;

use crate::example::_api::proxy::{ExampleProxyEvent, ExampleProxyInfra};

use crate::example::outline::remote::{
    get_menu_badge::data::OutlineMenuBadge,
    proxy_get_menu_badge::data::GetOutlineMenuBadgeProxyMessage,
};

pub struct GetOutlineMenuBadgeProxyStruct<'a> {
    pubsub: ActionStatePubSub<ExampleProxyEvent<GetOutlineMenuBadgeProxyMessage>>,
    validate_infra: ValidateApiMetadataStruct<'a>,
    proxy_service: ProxyService<'a>,
    response_encoder: ResponseEncoder,
}

impl<'a> GetOutlineMenuBadgeProxyStruct<'a> {
    pub fn new(feature: &'a ApiAppFeature, request_id: &'a str, request: &'a HttpRequest) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            validate_infra: ValidateApiMetadataStruct::new(&feature.auth.decoding_key, request),
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
    type ValidateInfra = ValidateApiMetadataStruct<'a>;
    type ProxyService = ProxyService<'a>;
    type ResponseEncoder = ResponseEncoder;

    fn validate_infra(&self) -> &Self::ValidateInfra {
        &self.validate_infra
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
