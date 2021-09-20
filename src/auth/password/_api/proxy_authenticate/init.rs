mod authenticate_service;
mod request_decoder;
mod response_encoder;

use actix_web::HttpRequest;

use getto_application::infra::ActionStatePubSub;

use crate::auth::password::_api::proxy_authenticate::infra::AuthenticatePasswordProxyRequestDecoder;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::kernel::init::auth_metadata::TicketAuthMetadata;
use authenticate_service::AuthenticateProxyService;
use request_decoder::AuthenticateRequestDecoder;
use response_encoder::AuthenticateResponseEncoder;

use crate::auth::_api::proxy::{AuthProxyMaterial, AuthProxyState};

use crate::auth::password::_common::authenticate::infra::AuthenticatePasswordFieldsExtract;

use crate::auth::{
    auth_ticket::_api::kernel::data::AuthTokenResponse,
    password::_api::proxy_authenticate::data::{
        AuthenticatePasswordResponse, AuthenticatePasswordResult,
    },
};

pub struct AuthenticatePasswordProxyFeature<'a> {
    pubsub: ActionStatePubSub<AuthProxyState<AuthenticatePasswordResult<AuthTokenResponse>>>,
    auth_metadata: TicketAuthMetadata<'a>,
    proxy_service: AuthenticateProxyService<'a>,
    response_encoder: AuthenticateResponseEncoder<'a>,
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
            proxy_service: AuthenticateProxyService::new(&feature.service, request_id),
            response_encoder: AuthenticateResponseEncoder::new(&feature.cookie),
        }
    }

    pub fn subscribe(
        &mut self,
        handler: impl 'static
            + Fn(&AuthProxyState<AuthenticatePasswordResult<AuthTokenResponse>>)
            + Send
            + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    pub fn request_decoder(body: String) -> impl AuthenticatePasswordProxyRequestDecoder {
        AuthenticateRequestDecoder::new(body)
    }
}

#[async_trait::async_trait]
impl<'a>
    AuthProxyMaterial<
        AuthenticatePasswordFieldsExtract,
        AuthenticatePasswordResponse,
        AuthenticatePasswordResult<AuthTokenResponse>,
    > for AuthenticatePasswordProxyFeature<'a>
{
    type AuthMetadata = TicketAuthMetadata<'a>;
    type ProxyService = AuthenticateProxyService<'a>;
    type ResponseEncoder = AuthenticateResponseEncoder<'a>;

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
        state: AuthProxyState<AuthenticatePasswordResult<AuthTokenResponse>>,
    ) -> AuthProxyState<AuthenticatePasswordResult<AuthTokenResponse>> {
        self.pubsub.post(state)
    }
}
