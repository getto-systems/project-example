mod proxy_service;
mod request_decoder;
mod response_encoder;

use actix_web::HttpRequest;

use getto_application::infra::ActionStatePubSub;

use crate::auth::remote::x_outside_feature::api::feature::AuthOutsideFeature;

use crate::auth::ticket::remote::validate_metadata::init::NoValidateMetadataStruct;
use proxy_service::ProxyService;
use request_decoder::RequestDecoder;
use response_encoder::ResponseEncoder;

use crate::auth::remote::service::proxy::{AuthProxyEvent, AuthProxyInfra};

use crate::auth::user::password::reset::remote::proxy_request_token::infra::{
    RequestResetTokenFieldsExtract, RequestResetTokenProxyRequestDecoder,
    RequestResetTokenProxyResponse,
};

use crate::auth::user::password::reset::remote::proxy_request_token::data::RequestResetTokenProxyMessage;

pub struct RequestResetTokenProxyStruct<'a> {
    pubsub: ActionStatePubSub<AuthProxyEvent<RequestResetTokenProxyMessage>>,
    validate_infra: NoValidateMetadataStruct<'a>,
    proxy_service: ProxyService<'a>,
    response_encoder: ResponseEncoder,
}

impl<'a> RequestResetTokenProxyStruct<'a> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            validate_infra: NoValidateMetadataStruct::new(request),
            proxy_service: ProxyService::new(&feature.service, request_id),
            response_encoder: ResponseEncoder,
        }
    }

    pub fn subscribe(
        &mut self,
        handler: impl 'static + Fn(&AuthProxyEvent<RequestResetTokenProxyMessage>) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    pub fn request_decoder(body: String) -> impl RequestResetTokenProxyRequestDecoder {
        RequestDecoder::new(body)
    }
}

#[async_trait::async_trait]
impl<'a>
    AuthProxyInfra<
        RequestResetTokenFieldsExtract,
        RequestResetTokenProxyResponse,
        RequestResetTokenProxyMessage,
    > for RequestResetTokenProxyStruct<'a>
{
    type ValidateInfra = NoValidateMetadataStruct<'a>;
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
        state: AuthProxyEvent<RequestResetTokenProxyMessage>,
    ) -> AuthProxyEvent<RequestResetTokenProxyMessage> {
        self.pubsub.post(state)
    }
}
