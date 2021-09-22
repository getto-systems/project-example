mod proxy_service;
mod request_decoder;
mod response_encoder;

use actix_web::HttpRequest;

use getto_application::infra::ActionStatePubSub;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::{
    _api::kernel::init::auth_metadata::NoAuthMetadata,
    _common::kernel::init::token_decoder::NoopTokenDecoder,
};
use proxy_service::ProxyService;
use request_decoder::RequestDecoder;
use response_encoder::ResponseEncoder;

use crate::auth::_api::proxy::{AuthProxyEvent, AuthProxyInfra};

use crate::auth::password::reset::{
    _api::proxy_request_token::infra::{
        RequestResetTokenProxyRequestDecoder, RequestResetTokenProxyResponse,
    },
    _common::request_token::infra::RequestResetTokenFieldsExtract,
};

use crate::auth::password::reset::_api::proxy_request_token::data::RequestResetTokenProxyMessage;

pub struct RequestResetTokenProxyStruct<'a> {
    pubsub: ActionStatePubSub<AuthProxyEvent<RequestResetTokenProxyMessage>>,
    auth_metadata: NoAuthMetadata<'a>,
    token_decoder: NoopTokenDecoder,
    proxy_service: ProxyService<'a>,
    response_encoder: ResponseEncoder,
}

impl<'a> RequestResetTokenProxyStruct<'a> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            auth_metadata: NoAuthMetadata::new(request),
            token_decoder: NoopTokenDecoder,
            proxy_service: ProxyService::new(&feature.service, request_id),
            response_encoder: ResponseEncoder,
        }
    }

    pub fn subscribe(
        &mut self,
        handler: impl 'static + Fn(&AuthProxyEvent<RequestResetTokenProxyMessage>) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    pub fn request_decoder(body: String) -> impl RequestResetTokenProxyRequestDecoder {
        RequestDecoder::new(body)
    }
}

#[async_trait::async_trait]
impl<'a>
    AuthProxyInfra<
        RequestResetTokenFieldsExtract,
        RequestResetTokenProxyResponse,
        RequestResetTokenProxyMessage,
    > for RequestResetTokenProxyStruct<'a>
{
    type AuthMetadata = NoAuthMetadata<'a>;
    type TokenDecoder = NoopTokenDecoder;
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
        state: AuthProxyEvent<RequestResetTokenProxyMessage>,
    ) -> AuthProxyEvent<RequestResetTokenProxyMessage> {
        self.pubsub.post(state)
    }
}
