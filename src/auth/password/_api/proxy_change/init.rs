mod proxy_service;
mod request_decoder;
mod response_encoder;

use actix_web::HttpRequest;

use getto_application::infra::ActionStatePubSub;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::_api::init::{ApiAuthMetadata, JwtApiTokenDecoder};
use proxy_service::ProxyService;
use request_decoder::RequestDecoder;
use response_encoder::ResponseEncoder;

use crate::auth::_api::proxy::{AuthProxyEvent, AuthProxyInfra};

use crate::auth::password::{
    _api::proxy_change::infra::{ChangePasswordProxyRequestDecoder, ChangePasswordProxyResponse},
    _common::change::infra::ChangePasswordFieldsExtract,
};

use crate::auth::password::_api::proxy_change::data::ChangePasswordProxyMessage;

pub struct ChangePasswordProxyStruct<'a> {
    pubsub: ActionStatePubSub<AuthProxyEvent<ChangePasswordProxyMessage>>,
    auth_metadata: ApiAuthMetadata<'a>,
    token_decoder: JwtApiTokenDecoder<'a>,
    proxy_service: ProxyService<'a>,
    response_encoder: ResponseEncoder,
}

impl<'a> ChangePasswordProxyStruct<'a> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            auth_metadata: ApiAuthMetadata::new(request),
            token_decoder: JwtApiTokenDecoder::new(&feature.decoding_key),
            proxy_service: ProxyService::new(&feature.service, request_id),
            response_encoder: ResponseEncoder,
        }
    }

    pub fn subscribe(
        &mut self,
        handler: impl 'static + Fn(&AuthProxyEvent<ChangePasswordProxyMessage>) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    pub fn request_decoder(body: String) -> impl ChangePasswordProxyRequestDecoder {
        RequestDecoder::new(body)
    }
}

#[async_trait::async_trait]
impl<'a>
    AuthProxyInfra<
        ChangePasswordFieldsExtract,
        ChangePasswordProxyResponse,
        ChangePasswordProxyMessage,
    > for ChangePasswordProxyStruct<'a>
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
        state: AuthProxyEvent<ChangePasswordProxyMessage>,
    ) -> AuthProxyEvent<ChangePasswordProxyMessage> {
        self.pubsub.post(state)
    }
}
