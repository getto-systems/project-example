mod proxy_service;
mod response_encoder;

use actix_web::HttpRequest;

use getto_application::infra::ActionStatePubSub;

use crate::auth::remote::x_outside_feature::api::feature::AuthOutsideFeature;

use crate::auth::ticket::remote::validate::init::ValidateTicketMetadataStruct;
use proxy_service::ProxyService;
use response_encoder::ResponseEncoder;

use crate::auth::remote::service::proxy::{AuthProxyEvent, AuthProxyInfra};

use crate::auth::ticket::remote::{
    encode::data::AuthTicketEncoded, kernel::data::AuthTokenResponse,
};

pub struct CheckAuthTicketProxyStruct<'a> {
    pubsub: ActionStatePubSub<AuthProxyEvent<AuthTokenResponse>>,
    validate_infra: ValidateTicketMetadataStruct<'a>,
    proxy_service: ProxyService<'a>,
    response_encoder: ResponseEncoder<'a>,
}

impl<'a> CheckAuthTicketProxyStruct<'a> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            validate_infra: ValidateTicketMetadataStruct::new(&feature.decoding_key, request),
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
    for CheckAuthTicketProxyStruct<'a>
{
    type ValidateInfra = ValidateTicketMetadataStruct<'a>;
    type ProxyService = ProxyService<'a>;
    type ResponseEncoder = ResponseEncoder<'a>;

    fn validate_infra(&self) -> &Self::ValidateInfra {
        &self.validate_infra
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
