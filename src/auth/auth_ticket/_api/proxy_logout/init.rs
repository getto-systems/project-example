mod proxy_service;
mod response_encoder;

use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::kernel::init::auth_metadata::TicketAuthMetadata;
use proxy_service::ProxyService;
use response_encoder::{LogoutProxyResponse, ResponseEncoder};

use crate::auth::_api::proxy::{AuthProxyMaterial, AuthProxyState};

use getto_application::infra::ActionStatePubSub;

pub struct LogoutProxyFeature<'a> {
    pubsub: ActionStatePubSub<AuthProxyState<LogoutProxyResponse>>,
    auth_metadata: TicketAuthMetadata<'a>,
    proxy_service: ProxyService<'a>,
    response_encoder: ResponseEncoder,
}

impl<'a> LogoutProxyFeature<'a> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            auth_metadata: TicketAuthMetadata::new(&feature.key, request),
            proxy_service: ProxyService::new(&feature.service, request_id),
            response_encoder: ResponseEncoder,
        }
    }

    pub fn subscribe(
        &mut self,
        handler: impl 'static + Fn(&AuthProxyState<LogoutProxyResponse>) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }
}

#[async_trait::async_trait]
impl<'a> AuthProxyMaterial<(), (), LogoutProxyResponse> for LogoutProxyFeature<'a> {
    type AuthMetadata = TicketAuthMetadata<'a>;
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
        state: AuthProxyState<LogoutProxyResponse>,
    ) -> AuthProxyState<LogoutProxyResponse> {
        self.pubsub.post(state)
    }
}
