mod proxy_service;
mod request_decoder;
mod response_encoder;

use actix_web::HttpRequest;

use getto_application::infra::ActionStatePubSub;

use crate::auth::remote::x_outside_feature::api::feature::AuthOutsideFeature;

use crate::auth::ticket::remote::validate_metadata::init::ValidateApiMetadataStruct;
use proxy_service::ProxyService;
use request_decoder::RequestDecoder;
use response_encoder::ResponseEncoder;

use crate::auth::remote::service::proxy::{AuthProxyEvent, AuthProxyInfra};

use crate::auth::user::account::remote::search::{
    infra::SearchAuthUserAccountFieldsExtract,
    proxy::infra::{SearchAuthUserAccountProxyRequestDecoder, SearchAuthUserAccountProxyResponse},
};

use crate::auth::user::account::remote::search::proxy::data::SearchAuthUserAccountProxyMessage;

pub struct SearchAuthUserAccountProxyStruct<'a> {
    pubsub: ActionStatePubSub<AuthProxyEvent<SearchAuthUserAccountProxyMessage>>,
    validate_infra: ValidateApiMetadataStruct<'a>,
    proxy_service: ProxyService<'a>,
    response_encoder: ResponseEncoder,
}

impl<'a> SearchAuthUserAccountProxyStruct<'a> {
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
        handler: impl 'static + Fn(&AuthProxyEvent<SearchAuthUserAccountProxyMessage>) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    pub fn request_decoder(body: String) -> impl SearchAuthUserAccountProxyRequestDecoder {
        RequestDecoder::new(body)
    }
}

#[async_trait::async_trait]
impl<'a>
    AuthProxyInfra<
        SearchAuthUserAccountFieldsExtract,
        SearchAuthUserAccountProxyResponse,
        SearchAuthUserAccountProxyMessage,
    > for SearchAuthUserAccountProxyStruct<'a>
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
        state: AuthProxyEvent<SearchAuthUserAccountProxyMessage>,
    ) -> AuthProxyEvent<SearchAuthUserAccountProxyMessage> {
        self.pubsub.post(state)
    }
}
