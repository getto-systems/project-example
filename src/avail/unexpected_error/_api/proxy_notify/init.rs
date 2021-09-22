mod proxy_service;
mod response_encoder;
mod request_decoder;

use actix_web::HttpRequest;

use getto_application::infra::ActionStatePubSub;

use crate::avail::unexpected_error::_api::proxy_notify::data::NotifyUnexpectedErrorProxyMessage;
use crate::avail::unexpected_error::_api::proxy_notify::infra::NotifyUnexpectedErrorProxyRequestDecoder;
use crate::avail::unexpected_error::_common::notify::infra::NotifyUnexpectedErrorFieldsExtract;
use crate::x_outside_feature::_api::feature::ApiAppFeature;

use crate::auth::_api::init::{ApiAuthMetadata, JwtApiTokenDecoder};
use proxy_service::ProxyService;
use response_encoder::ResponseEncoder;
use request_decoder::RequestDecoder;

use crate::example::_api::proxy::{ExampleProxyEvent, ExampleProxyInfra};

pub struct NotifyUnexpectedErrorProxyStruct<'a> {
    pubsub: ActionStatePubSub<ExampleProxyEvent<NotifyUnexpectedErrorProxyMessage>>,
    auth_metadata: ApiAuthMetadata<'a>,
    token_decoder: JwtApiTokenDecoder<'a>,
    proxy_service: ProxyService<'a>,
    response_encoder: ResponseEncoder,
}

impl<'a> NotifyUnexpectedErrorProxyStruct<'a> {
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
        handler: impl 'static + Fn(&ExampleProxyEvent<NotifyUnexpectedErrorProxyMessage>) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    pub fn request_decoder(body: String) -> impl NotifyUnexpectedErrorProxyRequestDecoder {
        RequestDecoder::new(body)
    }
}

#[async_trait::async_trait]
impl<'a> ExampleProxyInfra<NotifyUnexpectedErrorFieldsExtract, (), NotifyUnexpectedErrorProxyMessage>
    for NotifyUnexpectedErrorProxyStruct<'a>
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
        state: ExampleProxyEvent<NotifyUnexpectedErrorProxyMessage>,
    ) -> ExampleProxyEvent<NotifyUnexpectedErrorProxyMessage> {
        self.pubsub.post(state)
    }
}
