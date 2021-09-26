mod proxy_service;
mod request_decoder;
mod response_encoder;

use actix_web::HttpRequest;

use getto_application::infra::ActionStatePubSub;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::validate_metadata::init::NoValidateMetadataStruct;
use proxy_service::ProxyService;
use request_decoder::RequestDecoder;
use response_encoder::ResponseEncoder;

use crate::auth::_api::proxy::{AuthProxyEvent, AuthProxyInfra};

use crate::auth::password::reset::remote::proxy_reset::infra::{
    ResetPasswordFieldsExtract, ResetPasswordProxyRequestDecoder, ResetPasswordProxyResponse,
};

use crate::auth::password::reset::remote::proxy_reset::data::ResetPasswordProxyMessage;

pub struct ResetPasswordProxyStruct<'a> {
    pubsub: ActionStatePubSub<AuthProxyEvent<ResetPasswordProxyMessage>>,
    validate_infra: NoValidateMetadataStruct<'a>,
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
            validate_infra: NoValidateMetadataStruct::new(request),
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
    type ValidateInfra = NoValidateMetadataStruct<'a>;
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
        state: AuthProxyEvent<ResetPasswordProxyMessage>,
    ) -> AuthProxyEvent<ResetPasswordProxyMessage> {
        self.pubsub.post(state)
    }
}
