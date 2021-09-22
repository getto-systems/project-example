use actix_web::{HttpRequest, HttpResponse};

use getto_application::data::MethodResult;

use crate::z_details::_common::{
    logger::{LogLevel, LogMessage},
    request::data::MetadataError,
    response::actix_web::RespondTo,
};

use crate::auth::_common::infra::{AuthMetadata, AuthMetadataContent, AuthTokenDecoder};

use crate::{
    auth::_common::data::DecodeAuthTokenError, example::_api::service::data::ExampleServiceError,
    z_details::_api::message::data::MessageError,
};

pub enum ExampleProxyEvent<T> {
    TryToCall(String),
    Response(T),
    MetadataError(MetadataError),
    DecodeError(DecodeAuthTokenError),
    ServiceError(ExampleServiceError),
    MessageError(MessageError),
}

const SUCCESS: &'static str = "proxy call success";
const ERROR: &'static str = "proxy call error";

impl<T> std::fmt::Display for ExampleProxyEvent<T> {
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

impl<T: RespondTo> RespondTo for ExampleProxyEvent<T> {
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

impl<T> ExampleProxyEvent<T> {
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

impl<T> LogMessage for &ExampleProxyEvent<T> {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

pub trait ExampleProxyInfra<P, T, R> {
    type AuthMetadata: AuthMetadata;
    type TokenDecoder: AuthTokenDecoder;
    type ProxyService: ExampleProxyService<P, T>;
    type ResponseEncoder: ExampleProxyResponseEncoder<T, R>;

    fn auth_metadata(&self) -> &Self::AuthMetadata;
    fn token_decoder(&self) -> &Self::TokenDecoder;
    fn proxy_service(&self) -> &Self::ProxyService;
    fn response_encoder(&self) -> &Self::ResponseEncoder;

    fn post(&self, state: ExampleProxyEvent<R>) -> ExampleProxyEvent<R>;
}

#[async_trait::async_trait]
pub trait ExampleProxyService<P, T> {
    fn name(&self) -> &str;
    async fn call(
        &self,
        metadata: AuthMetadataContent,
        params: P,
    ) -> Result<T, ExampleServiceError>;
}

pub trait ExampleProxyResponseEncoder<T, R> {
    fn encode(&self, response: T) -> Result<R, MessageError>;
}

pub async fn call_proxy<P, T, R>(
    infra: &impl ExampleProxyInfra<P, T, R>,
    params: Result<P, MessageError>,
) -> MethodResult<ExampleProxyEvent<R>> {
    let auth_metadata = infra.auth_metadata();
    let token_decoder = infra.token_decoder();
    let proxy_service = infra.proxy_service();
    let response_encoder = infra.response_encoder();

    let params = params.map_err(|err| infra.post(ExampleProxyEvent::MessageError(err)))?;

    let metadata = auth_metadata
        .metadata()
        .map_err(|err| infra.post(ExampleProxyEvent::MetadataError(err)))?;

    if let Some(ref token) = metadata.token {
        token_decoder
            .decode(token)
            .map_err(|err| infra.post(ExampleProxyEvent::DecodeError(err)))?;
    }

    infra.post(ExampleProxyEvent::TryToCall(proxy_service.name().into()));

    let response = proxy_service
        .call(metadata, params)
        .await
        .map_err(|err| infra.post(ExampleProxyEvent::ServiceError(err)))?;

    let message = response_encoder
        .encode(response)
        .map_err(|err| infra.post(ExampleProxyEvent::MessageError(err)))?;

    Ok(infra.post(ExampleProxyEvent::Response(message)))
}
