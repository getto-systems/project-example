mod proxy_service;
mod request_decoder;
mod response_encoder;

use actix_web::HttpRequest;

use getto_application::infra::ActionStatePubSub;

use crate::auth::remote::x_outside_feature::api::feature::AuthOutsideFeature;

use crate::auth::ticket::remote::validate_metadata::init::ValidateTicketMetadataStruct;
use proxy_service::ProxyService;
use request_decoder::RequestDecoder;
use response_encoder::ResponseEncoder;

use crate::auth::remote::service::proxy::{AuthProxyEvent, AuthProxyInfra};

use crate::auth::user::account::remote::{
    proxy_search::infra::{SearchUserAccountProxyRequestDecoder, SearchUserAccountProxyResponse},
    search::infra::SearchUserAccountFieldsExtract,
};

use crate::auth::user::account::remote::proxy_search::data::SearchUserAccountProxyMessage;

pub struct SearchUserAccountProxyStruct<'a> {
    pubsub: ActionStatePubSub<AuthProxyEvent<SearchUserAccountProxyMessage>>,
    validate_infra: ValidateTicketMetadataStruct<'a>,
    proxy_service: ProxyService<'a>,
    response_encoder: ResponseEncoder,
}

impl<'a> SearchUserAccountProxyStruct<'a> {
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
        handler: impl 'static + Fn(&AuthProxyEvent<SearchUserAccountProxyMessage>) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    pub fn request_decoder(body: String) -> impl SearchUserAccountProxyRequestDecoder {
        RequestDecoder::new(body)
    }
}

#[async_trait::async_trait]
impl<'a>
    AuthProxyInfra<
        SearchUserAccountFieldsExtract,
        SearchUserAccountProxyResponse,
        SearchUserAccountProxyMessage,
    > for SearchUserAccountProxyStruct<'a>
{
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
        state: AuthProxyEvent<SearchUserAccountProxyMessage>,
    ) -> AuthProxyEvent<SearchUserAccountProxyMessage> {
        self.pubsub.post(state)
    }
}
