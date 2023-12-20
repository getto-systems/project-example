use std::sync::Arc;

use actix_web::{patch, web::Data, HttpRequest, HttpResponse};

use crate::{
    common::api::{
        feature::AsInfra, logger::detail::StdoutJsonLogger, response::x_actix_web::ProxyResponder,
    },
    x_outside_feature::proxy::feature::ProxyAppFeature,
};

use crate::auth::ticket::authenticate::{
    action::CheckAuthenticateTokenAction, proxy::action::AuthenticateWithTokenProxyAction,
};

use crate::{
    auth::ticket::authenticate::{
        data::CheckAuthenticateTokenError, proxy::data::AuthenticateWithTokenProxyError,
    },
    common::api::request::data::RequestInfo,
};

#[patch("")]
pub async fn service_authenticate_with_token(
    feature: Data<ProxyAppFeature>,
    request: HttpRequest,
) -> HttpResponse {
    async {
        let info = RequestInfo::from_request(&request);
        let logger = Arc::new(StdoutJsonLogger::with_request(info.clone()));

        let auth = CheckAuthenticateTokenAction::live(feature.as_infra())
            .with_logger(logger.clone())
            .check(&request)?;

        Ok::<_, AppError>(
            AuthenticateWithTokenProxyAction::live(feature.as_infra())
                .with_logger(logger.clone())
                .authenticate(info, &request, auth)
                .await?,
        )
    }
    .await
    .respond_to()
}

enum AppError {
    CheckAuthenticateTokenError(CheckAuthenticateTokenError),
    AuthenticateWithTokenError(AuthenticateWithTokenProxyError),
}

impl From<CheckAuthenticateTokenError> for AppError {
    fn from(value: CheckAuthenticateTokenError) -> Self {
        Self::CheckAuthenticateTokenError(value)
    }
}

impl From<AuthenticateWithTokenProxyError> for AppError {
    fn from(value: AuthenticateWithTokenProxyError) -> Self {
        Self::AuthenticateWithTokenError(value)
    }
}

impl ProxyResponder for AppError {
    fn respond_to(self) -> HttpResponse {
        match self {
            Self::CheckAuthenticateTokenError(err) => err.respond_to(),
            Self::AuthenticateWithTokenError(err) => err.respond_to(),
        }
    }
}

impl ProxyResponder for CheckAuthenticateTokenError {
    fn respond_to(self) -> HttpResponse {
        match self {
            Self::Invalid(err) => err.respond_to(),
            Self::DecodeError(err) => err.respond_to(),
        }
    }
}

impl ProxyResponder for AuthenticateWithTokenProxyError {
    fn respond_to(self) -> HttpResponse {
        match self {
            Self::Invalid(err) => err.respond_to(),
            Self::ProxyError(err) => err.respond_to(),
        }
    }
}
