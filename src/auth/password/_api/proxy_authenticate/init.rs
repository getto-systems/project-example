mod proxy_service;
mod request_decoder;
mod response_encoder;

use actix_web::HttpRequest;

use getto_application::infra::ActionStatePubSub;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::kernel::init::auth_metadata::TicketAuthMetadata;
use proxy_service::ProxyService;
use request_decoder::RequestDecoder;
use response_encoder::ResponseEncoder;

use crate::auth::_api::proxy::{AuthProxyMaterial, AuthProxyState};

use crate::auth::password::{
    _api::proxy_authenticate::infra::{
        AuthenticatePasswordProxyRequestDecoder, AuthenticatePasswordProxyResponse,
    },
    _common::authenticate::infra::AuthenticatePasswordFieldsExtract,
};

use crate::auth::password::_api::proxy_authenticate::data::AuthenticatePasswordProxyMessage;

pub struct AuthenticatePasswordProxyFeature<'a> {
    pubsub: ActionStatePubSub<AuthProxyState<AuthenticatePasswordProxyMessage>>,
    auth_metadata: TicketAuthMetadata<'a>,
    proxy_service: ProxyService<'a>,
    response_encoder: ResponseEncoder<'a>,
}

impl<'a> AuthenticatePasswordProxyFeature<'a> {
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
        handler: impl 'static + Fn(&AuthProxyState<AuthenticatePasswordProxyMessage>) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    pub fn request_decoder(body: String) -> impl AuthenticatePasswordProxyRequestDecoder {
        RequestDecoder::new(body)
    }
}

#[async_trait::async_trait]
impl<'a>
    AuthProxyMaterial<
        AuthenticatePasswordFieldsExtract,
        AuthenticatePasswordProxyResponse,
        AuthenticatePasswordProxyMessage,
    > for AuthenticatePasswordProxyFeature<'a>
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

    fn post(
        &self,
        state: AuthProxyState<AuthenticatePasswordProxyMessage>,
    ) -> AuthProxyState<AuthenticatePasswordProxyMessage> {
        self.pubsub.post(state)
    }
}
