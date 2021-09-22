mod proxy_service;
mod response_encoder;

use actix_web::HttpRequest;

use getto_application::infra::ActionStatePubSub;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::{
    _api::kernel::init::auth_metadata::TicketAuthMetadata,
    _common::kernel::init::token_decoder::JwtTicketTokenDecoder,
};
use proxy_service::ProxyService;
use response_encoder::ResponseEncoder;

use crate::auth::_api::proxy::{AuthProxyEvent, AuthProxyInfra};

use crate::auth::auth_ticket::{
    _api::kernel::data::AuthTokenResponse, _common::encode::data::AuthTicketEncoded,
};

pub struct RenewAuthTicketProxyStruct<'a> {
    pubsub: ActionStatePubSub<AuthProxyEvent<AuthTokenResponse>>,
    auth_metadata: TicketAuthMetadata<'a>,
    token_decoder: JwtTicketTokenDecoder<'a>,
    proxy_service: ProxyService<'a>,
    response_encoder: ResponseEncoder<'a>,
}

impl<'a> RenewAuthTicketProxyStruct<'a> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            auth_metadata: TicketAuthMetadata::new(request),
            token_decoder: JwtTicketTokenDecoder::new(&feature.decoding_key),
            proxy_service: ProxyService::new(&feature.service, request_id),
            response_encoder: ResponseEncoder::new(&feature.cookie),
        }
    }

    pub fn subscribe(
        &mut self,
        handler: impl 'static + Fn(&AuthProxyEvent<AuthTokenResponse>) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }
}

#[async_trait::async_trait]
impl<'a> AuthProxyInfra<(), AuthTicketEncoded, AuthTokenResponse>
    for RenewAuthTicketProxyStruct<'a>
{
    type AuthMetadata = TicketAuthMetadata<'a>;
    type TokenDecoder = JwtTicketTokenDecoder<'a>;
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

    fn post(&self, state: AuthProxyEvent<AuthTokenResponse>) -> AuthProxyEvent<AuthTokenResponse> {
        self.pubsub.post(state)
    }
}
