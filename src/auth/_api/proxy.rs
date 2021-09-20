use actix_web::{HttpRequest, HttpResponse};

use getto_application::data::MethodResult;

use crate::z_details::_common::{
    logger::{LogLevel, LogMessage},
    response::actix_web::RespondTo,
};

use crate::auth::_common::infra::{AuthMetadata, AuthMetadataContent};

use crate::{
    auth::_common::{data::AuthMetadataError, service::data::AuthServiceError},
    z_details::_api::message::data::MessageError,
};

pub enum AuthProxyState<T> {
    TryToCall(String),
    Response(T),
    MetadataError(AuthMetadataError),
    ServiceError(AuthServiceError),
    MessageError(MessageError),
}

const SUCCESS: &'static str = "proxy call success";
const ERROR: &'static str = "proxy call error";

impl<T> std::fmt::Display for AuthProxyState<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TryToCall(target) => write!(f, "try to proxy call: {}", target),
            Self::Response(_) => write!(f, "{}", SUCCESS),
            Self::MetadataError(err) => write!(f, "{}; {}", ERROR, err),
            Self::ServiceError(err) => write!(f, "{}; {}", ERROR, err),
            Self::MessageError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

impl<T: RespondTo> RespondTo for AuthProxyState<T> {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::TryToCall(_) => HttpResponse::Accepted().finish(),
            Self::Response(response) => response.respond_to(request),
            Self::MetadataError(err) => err.respond_to(request),
            Self::ServiceError(err) => err.respond_to(request),
            Self::MessageError(err) => err.respond_to(request),
        }
    }
}

impl<T> AuthProxyState<T> {
    pub fn log_level(&self) -> LogLevel {
        match self {
            Self::TryToCall(_) => LogLevel::Debug,
            Self::Response(_) => LogLevel::Info,
            Self::MetadataError(_) => LogLevel::Error,
            Self::ServiceError(_) => LogLevel::Error,
            Self::MessageError(_) => LogLevel::Error,
        }
    }
}

impl<T> LogMessage for &AuthProxyState<T> {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

pub trait AuthProxyMaterial<P, T, R> {
    type AuthMetadata: AuthMetadata;
    type ProxyService: AuthProxyService<P, T>;
    type ResponseEncoder: AuthProxyResponseEncoder<T, R>;

    fn auth_metadata(&self) -> &Self::AuthMetadata;
    fn proxy_service(&self) -> &Self::ProxyService;
    fn response_encoder(&self) -> &Self::ResponseEncoder;

    fn post(&self, state: AuthProxyState<R>) -> AuthProxyState<R>;
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
    material: &impl AuthProxyMaterial<P, T, R>,
    params: Result<P, MessageError>,
) -> MethodResult<AuthProxyState<R>> {
    let auth_metadata = material.auth_metadata();
    let proxy_service = material.proxy_service();
    let response_encoder = material.response_encoder();

    let params = params.map_err(|err| material.post(AuthProxyState::MessageError(err)))?;

    let metadata = auth_metadata
        .metadata()
        .map_err(|err| material.post(AuthProxyState::MetadataError(err)))?;

    material.post(AuthProxyState::TryToCall(proxy_service.name().into()));

    let response = proxy_service
        .call(metadata, params)
        .await
        .map_err(|err| material.post(AuthProxyState::ServiceError(err)))?;

    let message = response_encoder
        .encode(response)
        .map_err(|err| material.post(AuthProxyState::MessageError(err)))?;

    Ok(material.post(AuthProxyState::Response(message)))
}
