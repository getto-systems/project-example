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
    _api::proxy_reset::infra::{ResetPasswordProxyRequestDecoder, ResetPasswordProxyResponse},
    _common::reset::infra::ResetPasswordFieldsExtract,
};

use crate::auth::password::reset::_api::proxy_reset::data::ResetPasswordProxyMessage;

pub struct ResetPasswordProxyStruct<'a> {
    pubsub: ActionStatePubSub<AuthProxyEvent<ResetPasswordProxyMessage>>,
    auth_metadata: NoAuthMetadata<'a>,
    token_decoder: NoopTokenDecoder,
    proxy_service: ProxyService<'a>,
    response_encoder: ResponseEncoder<'a>,
}

impl<'a> ResetPasswordProxyStruct<'a> {
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
            response_encoder: ResponseEncoder::new(&feature.cookie),
        }
    }

    pub fn subscribe(
        &mut self,
        handler: impl 'static + Fn(&AuthProxyEvent<ResetPasswordProxyMessage>) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    pub fn request_decoder(body: String) -> impl ResetPasswordProxyRequestDecoder {
        RequestDecoder::new(body)
    }
}

#[async_trait::async_trait]
impl<'a>
    AuthProxyInfra<
        ResetPasswordFieldsExtract,
        ResetPasswordProxyResponse,
        ResetPasswordProxyMessage,
    > for ResetPasswordProxyStruct<'a>
{
    type AuthMetadata = NoAuthMetadata<'a>;
    type TokenDecoder = NoopTokenDecoder;
    type ProxyService = ProxyService<'a>;
    type ResponseEncoder = ResponseEncoder<'a>;

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
        state: AuthProxyEvent<ResetPasswordProxyMessage>,
    ) -> AuthProxyEvent<ResetPasswordProxyMessage> {
        self.pubsub.post(state)
    }
}
