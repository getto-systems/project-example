mod change_service;
mod request_decoder;
mod response_encoder;

use actix_web::HttpRequest;

use getto_application::infra::ActionStatePubSub;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::_api::init::ApiAuthMetadata;
use change_service::ChangeProxyService;
use request_decoder::ChangeProxyRequestDecoder;
use response_encoder::ChangeProxyResponseEncoder;

use crate::auth::_api::proxy::{AuthProxyMaterial, AuthProxyState};

use crate::auth::password::{
    _api::proxy_change::infra::{ChangePasswordProxyRequestDecoder, ChangePasswordProxyResponse},
    _common::change::infra::ChangePasswordFieldsExtract,
};

use crate::auth::password::_api::proxy_change::data::ChangePasswordProxyMessage;

pub struct ChangePasswordProxyFeature<'a> {
    pubsub: ActionStatePubSub<AuthProxyState<ChangePasswordProxyMessage>>,
    auth_metadata: ApiAuthMetadata<'a>,
    proxy_service: ChangeProxyService<'a>,
    response_encoder: ChangeProxyResponseEncoder,
}

impl<'a> ChangePasswordProxyFeature<'a> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            auth_metadata: ApiAuthMetadata::new(&feature.key, request),
            proxy_service: ChangeProxyService::new(&feature.service, request_id),
            response_encoder: ChangeProxyResponseEncoder,
        }
    }

    pub fn subscribe(
        &mut self,
        handler: impl 'static + Fn(&AuthProxyState<ChangePasswordProxyMessage>) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    pub fn request_decoder(body: String) -> impl ChangePasswordProxyRequestDecoder {
        ChangeProxyRequestDecoder::new(body)
    }
}

#[async_trait::async_trait]
impl<'a>
    AuthProxyMaterial<
        ChangePasswordFieldsExtract,
        ChangePasswordProxyResponse,
        ChangePasswordProxyMessage,
    > for ChangePasswordProxyFeature<'a>
{
    type AuthMetadata = ApiAuthMetadata<'a>;
    type ProxyService = ChangeProxyService<'a>;
    type ResponseEncoder = ChangeProxyResponseEncoder;

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
        state: AuthProxyState<ChangePasswordProxyMessage>,
    ) -> AuthProxyState<ChangePasswordProxyMessage> {
        self.pubsub.post(state)
    }
}
