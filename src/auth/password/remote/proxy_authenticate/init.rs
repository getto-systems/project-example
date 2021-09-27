mod proxy_service;
mod request_decoder;
mod response_encoder;

use actix_web::HttpRequest;

use getto_application::infra::ActionStatePubSub;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::ticket::remote::validate_metadata::init::ValidateTicketMetadataStruct;
use proxy_service::ProxyService;
use request_decoder::RequestDecoder;
use response_encoder::ResponseEncoder;

use crate::auth::_api::proxy::{AuthProxyEvent, AuthProxyInfra};

use crate::auth::password::remote::proxy_authenticate::infra::{
    AuthenticatePasswordFieldsExtract, AuthenticatePasswordProxyRequestDecoder,
    AuthenticatePasswordProxyResponse,
};

use crate::auth::password::remote::proxy_authenticate::data::AuthenticatePasswordProxyMessage;

pub struct AuthenticatePasswordProxyStruct<'a> {
    pubsub: ActionStatePubSub<AuthProxyEvent<AuthenticatePasswordProxyMessage>>,
    validate_infra: ValidateTicketMetadataStruct<'a>,
    proxy_service: ProxyService<'a>,
    response_encoder: ResponseEncoder<'a>,
}

impl<'a> AuthenticatePasswordProxyStruct<'a> {
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
        handler: impl 'static + Fn(&AuthProxyEvent<AuthenticatePasswordProxyMessage>) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    pub fn request_decoder(body: String) -> impl AuthenticatePasswordProxyRequestDecoder {
        RequestDecoder::new(body)
    }
}

#[async_trait::async_trait]
impl<'a>
    AuthProxyInfra<
        AuthenticatePasswordFieldsExtract,
        AuthenticatePasswordProxyResponse,
        AuthenticatePasswordProxyMessage,
    > for AuthenticatePasswordProxyStruct<'a>
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

    fn post(
        &self,
        state: AuthProxyEvent<AuthenticatePasswordProxyMessage>,
    ) -> AuthProxyEvent<AuthenticatePasswordProxyMessage> {
        self.pubsub.post(state)
    }
}
