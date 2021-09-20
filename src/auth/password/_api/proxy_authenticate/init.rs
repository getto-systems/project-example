mod authenticate_service;
mod request_decoder;
mod response_encoder;

use actix_web::{HttpRequest, HttpResponse};

use getto_application::infra::ActionStatePubSub;

use crate::z_details::_common::response::actix_web::RespondTo;

use crate::auth::password::_common::y_protobuf::service::AuthenticatePasswordResponsePb;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_api::kernel::init::auth_metadata::TicketAuthMetadata;
use authenticate_service::AuthenticateProxyService;
use request_decoder::AuthenticateRequestDecoder;
use response_encoder::AuthenticateResponseEncoder;

use crate::auth::_api::proxy::{AuthProxyMaterial, AuthProxyState};

use crate::auth::password::_common::authenticate::infra::AuthenticatePasswordFieldsExtract;

use crate::{
    auth::{
        auth_ticket::{
            _api::kernel::data::{AuthTokenMessage, AuthTokenResponse},
            _common::{encode::data::AuthTicketEncoded, kernel::data::AuthTokenEncoded},
        },
        auth_user::_common::kernel::data::AuthUserExtract,
    },
    z_details::_api::message::data::MessageError,
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

    pub fn request_decoder(body: String) -> impl AuthenticatePasswordRequestDecoder {
        AuthenticateRequestDecoder::new(body)
    }
}

pub trait AuthenticatePasswordRequestDecoder {
    fn decode(self) -> Result<AuthenticatePasswordFieldsExtract, MessageError>;
}

pub type AuthenticatePasswordMessage = AuthenticatePasswordResult<AuthTokenResponse>;
pub type AuthenticatePasswordMessageEncoded = AuthenticatePasswordResult<AuthTokenMessage>;

impl RespondTo for AuthenticatePasswordMessage {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Success(message) => message.respond_to(request),
            Self::InvalidPassword(message) => HttpResponse::Ok().body(message),
        }
    }
}

pub enum AuthenticatePasswordResult<T> {
    Success(T),
    InvalidPassword(String),
}

impl<T> AuthenticatePasswordResult<T> {
    pub fn map<M>(self, mapper: impl Fn(T) -> M) -> AuthenticatePasswordResult<M> {
        match self {
            Self::InvalidPassword(response) => {
                AuthenticatePasswordResult::InvalidPassword(response)
            }
            Self::Success(response) => AuthenticatePasswordResult::Success(mapper(response)),
        }
    }
}

pub enum AuthenticatePasswordResponse {
    Success(AuthTicketEncoded),
    InvalidPassword,
}

impl Into<Option<AuthenticatePasswordResponse>> for AuthenticatePasswordResponsePb {
    fn into(self) -> Option<AuthenticatePasswordResponse> {
        if self.success {
            match (self.user, self.token) {
                (Some(user), Some(token)) => {
                    let user: AuthUserExtract = user.into();
                    let token: Option<AuthTokenEncoded> = token.into();
                    token.map(|token| {
                        AuthenticatePasswordResponse::Success(AuthTicketEncoded { user, token })
                    })
                }
                _ => None,
            }
        } else {
            Some(AuthenticatePasswordResponse::InvalidPassword)
        }
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
