mod proxy_service;
mod response_encoder;

use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::kernel::init::auth_metadata::TicketAuthMetadata;
use proxy_service::ProxyService;
use response_encoder::ResponseEncoder;

use crate::auth::_api::proxy::{AuthProxyMaterial, AuthProxyState};

use getto_application::infra::ActionStatePubSub;

use crate::auth::auth_ticket::{
    _api::kernel::data::AuthTokenResponse, _common::encode::data::AuthTicketEncoded,
};

pub struct RenewAuthTicketProxyFeature<'a> {
    pubsub: ActionStatePubSub<AuthProxyState<AuthTokenResponse>>,
    auth_metadata: TicketAuthMetadata<'a>,
    proxy_service: ProxyService<'a>,
    response_encoder: ResponseEncoder<'a>,
}

impl<'a> RenewAuthTicketProxyFeature<'a> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            auth_metadata: TicketAuthMetadata::new(&feature.key, request),
            proxy_service: ProxyService::new(&feature.service, request_id),
            response_encoder: ResponseEncoder::new(&feature.cookie),
        }
    }

    pub fn subscribe(
        &mut self,
        handler: impl 'static + Fn(&AuthProxyState<AuthTokenResponse>) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }
}

#[async_trait::async_trait]
impl<'a> AuthProxyMaterial<(), AuthTicketEncoded, AuthTokenResponse>
    for RenewAuthTicketProxyFeature<'a>
{
    type AuthMetadata = TicketAuthMetadata<'a>;
    type ProxyService = ProxyService<'a>;
    type ResponseEncoder = ResponseEncoder<'a>;

    fn auth_metadata(&self) -> &Self::AuthMetadata {
        &self.auth_metadata
    }
    fn proxy_service(&self) -> &Self::ProxyService {
        &self.proxy_service
    }
    fn response_encoder(&self) -> &Self::ResponseEncoder {
        &self.response_encoder
    }

    fn post(&self, state: AuthProxyState<AuthTokenResponse>) -> AuthProxyState<AuthTokenResponse> {
        self.pubsub.post(state)
    }
}
