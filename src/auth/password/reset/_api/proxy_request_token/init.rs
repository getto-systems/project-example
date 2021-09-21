mod proxy_service;
mod request_decoder;
mod response_encoder;

use actix_web::HttpRequest;

use getto_application::infra::ActionStatePubSub;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::kernel::init::auth_metadata::NoAuthMetadata;
use crate::auth::password::reset::_common::request_token::infra::RequestResetTokenFieldsExtract;
use proxy_service::ProxyService;
use request_decoder::RequestDecoder;
use response_encoder::ResponseEncoder;

use crate::auth::_api::proxy::{AuthProxyMaterial, AuthProxyState};

use crate::auth::password::reset::_api::proxy_request_token::infra::{
    RequestResetTokenProxyRequestDecoder, RequestResetTokenProxyResponse,
};

use crate::auth::password::reset::_api::proxy_request_token::data::RequestResetTokenProxyMessage;

pub struct RequestResetTokenProxyFeature<'a> {
    pubsub: ActionStatePubSub<AuthProxyState<RequestResetTokenProxyMessage>>,
    auth_metadata: NoAuthMetadata<'a>,
    proxy_service: ProxyService<'a>,
    response_encoder: ResponseEncoder,
}

impl<'a> RequestResetTokenProxyFeature<'a> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            auth_metadata: NoAuthMetadata::new(request),
            proxy_service: ProxyService::new(&feature.service, request_id),
            response_encoder: ResponseEncoder,
        }
    }

    pub fn subscribe(
        &mut self,
        handler: impl 'static + Fn(&AuthProxyState<RequestResetTokenProxyMessage>) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    pub fn request_decoder(body: String) -> impl RequestResetTokenProxyRequestDecoder {
        RequestDecoder::new(body)
    }
}

#[async_trait::async_trait]
impl<'a>
    AuthProxyMaterial<
        RequestResetTokenFieldsExtract,
        RequestResetTokenProxyResponse,
        RequestResetTokenProxyMessage,
    > for RequestResetTokenProxyFeature<'a>
{
    type AuthMetadata = NoAuthMetadata<'a>;
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
        state: AuthProxyState<RequestResetTokenProxyMessage>,
    ) -> AuthProxyState<RequestResetTokenProxyMessage> {
        self.pubsub.post(state)
    }
}
