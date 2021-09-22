mod proxy_service;
mod request_decoder;
mod response_encoder;

use actix_web::HttpRequest;

use getto_application::infra::ActionStatePubSub;

use crate::x_outside_feature::_api::feature::ApiAppFeature;

use crate::auth::_api::init::ValidateApiMetadataStruct;
use proxy_service::ProxyService;
use request_decoder::RequestDecoder;
use response_encoder::ResponseEncoder;

use crate::example::_api::proxy::{ExampleProxyEvent, ExampleProxyInfra};

use crate::avail::unexpected_error::{
    _api::proxy_notify::infra::NotifyUnexpectedErrorProxyRequestDecoder,
    _common::notify::infra::NotifyUnexpectedErrorFieldsExtract,
};

use crate::avail::unexpected_error::_api::proxy_notify::data::NotifyUnexpectedErrorProxyMessage;

pub struct NotifyUnexpectedErrorProxyStruct<'a> {
    pubsub: ActionStatePubSub<ExampleProxyEvent<NotifyUnexpectedErrorProxyMessage>>,
    validate_infra: ValidateApiMetadataStruct<'a>,
    proxy_service: ProxyService<'a>,
    response_encoder: ResponseEncoder,
}

impl<'a> NotifyUnexpectedErrorProxyStruct<'a> {
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
        handler: impl 'static + Fn(&ExampleProxyEvent<NotifyUnexpectedErrorProxyMessage>) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    pub fn request_decoder(body: String) -> impl NotifyUnexpectedErrorProxyRequestDecoder {
        RequestDecoder::new(body)
    }
}

#[async_trait::async_trait]
impl<'a>
    ExampleProxyInfra<NotifyUnexpectedErrorFieldsExtract, (), NotifyUnexpectedErrorProxyMessage>
    for NotifyUnexpectedErrorProxyStruct<'a>
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
        state: ExampleProxyEvent<NotifyUnexpectedErrorProxyMessage>,
    ) -> ExampleProxyEvent<NotifyUnexpectedErrorProxyMessage> {
        self.pubsub.post(state)
    }
}
