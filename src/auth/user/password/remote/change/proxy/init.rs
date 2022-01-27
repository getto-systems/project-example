mod proxy_service;
mod request_decoder;
mod response_encoder;

use actix_web::HttpRequest;

use getto_application::infra::ActionStatePubSub;

use crate::auth::remote::x_outside_feature::api::feature::AuthOutsideFeature;

use crate::auth::ticket::remote::validate::init::ValidateApiMetadataStruct;
use proxy_service::ProxyService;
use request_decoder::RequestDecoder;
use response_encoder::ResponseEncoder;

use crate::auth::remote::service::proxy::{AuthProxyEvent, AuthProxyInfra};

use crate::auth::user::password::remote::change::{
    infra::ChangePasswordFieldsExtract,
    proxy::infra::{ChangePasswordProxyRequestDecoder, ChangePasswordProxyResponse},
};

use crate::auth::user::password::remote::change::proxy::data::ChangePasswordProxyMessage;

pub struct ChangePasswordProxyStruct<'a> {
    pubsub: ActionStatePubSub<AuthProxyEvent<ChangePasswordProxyMessage>>,
    validate_infra: ValidateApiMetadataStruct<'a>,
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
            validate_infra: ValidateApiMetadataStruct::new(&feature.decoding_key, request),
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
    type ValidateInfra = ValidateApiMetadataStruct<'a>;
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
        state: AuthProxyEvent<ChangePasswordProxyMessage>,
    ) -> AuthProxyEvent<ChangePasswordProxyMessage> {
        self.pubsub.post(state)
    }
}
