mod proxy_service;
mod response_encoder;

use actix_web::HttpRequest;

use getto_application::infra::ActionStatePubSub;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::ticket::remote::validate_metadata::init::ValidateTicketMetadataStruct;
use proxy_service::ProxyService;
use response_encoder::{LogoutProxyResponse, ResponseEncoder};

use crate::auth::_api::proxy::{AuthProxyEvent, AuthProxyInfra};

pub struct LogoutProxyStruct<'a> {
    pubsub: ActionStatePubSub<AuthProxyEvent<LogoutProxyResponse>>,
    validate_infra: ValidateTicketMetadataStruct<'a>,
    proxy_service: ProxyService<'a>,
    response_encoder: ResponseEncoder,
}

impl<'a> LogoutProxyStruct<'a> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            validate_infra: ValidateTicketMetadataStruct::new(&feature.decoding_key, request),
            proxy_service: ProxyService::new(&feature.service, request_id),
            response_encoder: ResponseEncoder,
        }
    }

    pub fn subscribe(
        &mut self,
        handler: impl 'static + Fn(&AuthProxyEvent<LogoutProxyResponse>) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }
}

#[async_trait::async_trait]
impl<'a> AuthProxyInfra<(), (), LogoutProxyResponse> for LogoutProxyStruct<'a> {
    type ValidateInfra = ValidateTicketMetadataStruct<'a>;
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
        state: AuthProxyEvent<LogoutProxyResponse>,
    ) -> AuthProxyEvent<LogoutProxyResponse> {
        self.pubsub.post(state)
    }
}
