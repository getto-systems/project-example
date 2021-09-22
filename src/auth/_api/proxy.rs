use actix_web::{HttpRequest, HttpResponse};

use getto_application::data::MethodResult;

use crate::z_details::_common::{
    logger::{LogLevel, LogMessage},
    response::actix_web::RespondTo,
};

use crate::auth::_common::infra::{AuthMetadata, AuthMetadataContent, AuthTokenDecoder};

use crate::{
    auth::_common::{data::DecodeAuthTokenError, service::data::AuthServiceError},
    z_details::{_api::message::data::MessageError, _common::request::data::MetadataError},
};

pub enum AuthProxyEvent<T> {
    TryToCall(String),
    Response(T),
    MetadataError(MetadataError),
    DecodeError(DecodeAuthTokenError),
    ServiceError(AuthServiceError),
    MessageError(MessageError),
}

const SUCCESS: &'static str = "proxy call success";
const ERROR: &'static str = "proxy call error";

impl<T> std::fmt::Display for AuthProxyEvent<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TryToCall(target) => write!(f, "try to proxy call: {}", target),
            Self::Response(_) => write!(f, "{}", SUCCESS),
            Self::MetadataError(err) => write!(f, "{}; {}", ERROR, err),
            Self::DecodeError(err) => write!(f, "{}; {}", ERROR, err),
            Self::ServiceError(err) => write!(f, "{}; {}", ERROR, err),
            Self::MessageError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

impl<T: RespondTo> RespondTo for AuthProxyEvent<T> {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::TryToCall(_) => HttpResponse::Accepted().finish(),
            Self::Response(response) => response.respond_to(request),
            Self::MetadataError(err) => err.respond_to(request),
            Self::DecodeError(err) => err.respond_to(request),
            Self::ServiceError(err) => err.respond_to(request),
            Self::MessageError(err) => err.respond_to(request),
        }
    }
}

impl<T> AuthProxyEvent<T> {
    pub fn log_level(&self) -> LogLevel {
        match self {
            Self::TryToCall(_) => LogLevel::Info,
            Self::Response(_) => LogLevel::Debug,
            Self::MetadataError(err) => err.log_level(),
            Self::DecodeError(err) => err.log_level(),
            Self::ServiceError(err) => err.log_level(),
            Self::MessageError(err) => err.log_level(),
        }
    }
}

impl<T> LogMessage for &AuthProxyEvent<T> {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

pub trait AuthProxyInfra<P, T, R> {
    type AuthMetadata: AuthMetadata;
    type TokenDecoder: AuthTokenDecoder;
    type ProxyService: AuthProxyService<P, T>;
    type ResponseEncoder: AuthProxyResponseEncoder<T, R>;

    fn auth_metadata(&self) -> &Self::AuthMetadata;
    fn token_decoder(&self) -> &Self::TokenDecoder;
    fn proxy_service(&self) -> &Self::ProxyService;
    fn response_encoder(&self) -> &Self::ResponseEncoder;

    fn post(&self, state: AuthProxyEvent<R>) -> AuthProxyEvent<R>;
}

#[async_trait::async_trait]
pub trait AuthProxyService<P, T> {
    fn name(&self) -> &str;
    async fn call(&self, metadata: AuthMetadataContent, params: P) -> Result<T, AuthServiceError>;
}

pub trait AuthProxyResponseEncoder<T, R> {
    fn encode(&self, response: T) -> Result<R, MessageError>;
}

pub async fn call_proxy<P, T, R>(
    infra: &impl AuthProxyInfra<P, T, R>,
    params: Result<P, MessageError>,
) -> MethodResult<AuthProxyEvent<R>> {
    let auth_metadata = infra.auth_metadata();
    let token_decoder = infra.token_decoder();
    let proxy_service = infra.proxy_service();
    let response_encoder = infra.response_encoder();

    let params = params.map_err(|err| infra.post(AuthProxyEvent::MessageError(err)))?;

    let metadata = auth_metadata
        .metadata()
        .map_err(|err| infra.post(AuthProxyEvent::MetadataError(err)))?;

    if let Some(ref token) = metadata.token {
        token_decoder
            .decode(token)
            .map_err(|err| infra.post(AuthProxyEvent::DecodeError(err)))?;
    }

    infra.post(AuthProxyEvent::TryToCall(proxy_service.name().into()));

    let response = proxy_service
        .call(metadata, params)
        .await
        .map_err(|err| infra.post(AuthProxyEvent::ServiceError(err)))?;

    let message = response_encoder
        .encode(response)
        .map_err(|err| infra.post(AuthProxyEvent::MessageError(err)))?;

    Ok(infra.post(AuthProxyEvent::Response(message)))
}
