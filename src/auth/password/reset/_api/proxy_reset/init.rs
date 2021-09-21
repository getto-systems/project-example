mod proxy_service;
mod request_decoder;
mod response_encoder;

use actix_web::HttpRequest;

use getto_application::infra::ActionStatePubSub;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::kernel::init::auth_metadata::NoAuthMetadata;
use proxy_service::ProxyService;
use request_decoder::RequestDecoder;
use response_encoder::ResponseEncoder;

use crate::auth::_api::proxy::{AuthProxyMaterial, AuthProxyState};

use crate::auth::password::reset::{
    _api::proxy_reset::infra::{ResetPasswordProxyRequestDecoder, ResetPasswordProxyResponse},
    _common::reset::infra::ResetPasswordFieldsExtract,
};

use crate::auth::password::reset::_api::proxy_reset::data::ResetPasswordProxyMessage;

pub struct ResetPasswordProxyFeature<'a> {
    pubsub: ActionStatePubSub<AuthProxyState<ResetPasswordProxyMessage>>,
    auth_metadata: NoAuthMetadata<'a>,
    proxy_service: ProxyService<'a>,
    response_encoder: ResponseEncoder<'a>,
}

impl<'a> ResetPasswordProxyFeature<'a> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            auth_metadata: NoAuthMetadata::new(request),
            proxy_service: ProxyService::new(&feature.service, request_id),
            response_encoder: ResponseEncoder::new(&feature.cookie),
        }
    }

    pub fn subscribe(
        &mut self,
        handler: impl 'static + Fn(&AuthProxyState<ResetPasswordProxyMessage>) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    pub fn request_decoder(body: String) -> impl ResetPasswordProxyRequestDecoder {
        RequestDecoder::new(body)
    }
}

#[async_trait::async_trait]
impl<'a>
    AuthProxyMaterial<
        ResetPasswordFieldsExtract,
        ResetPasswordProxyResponse,
        ResetPasswordProxyMessage,
    > for ResetPasswordProxyFeature<'a>
{
    type AuthMetadata = NoAuthMetadata<'a>;
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
        state: AuthProxyState<ResetPasswordProxyMessage>,
    ) -> AuthProxyState<ResetPasswordProxyMessage> {
        self.pubsub.post(state)
    }
}
