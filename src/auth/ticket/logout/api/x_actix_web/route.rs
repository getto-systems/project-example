use std::sync::Arc;

use actix_web::{delete, web::Data, HttpRequest, HttpResponse};

use crate::{
    common::api::{
        feature::AsInfra, logger::detail::StdoutJsonLogger, response::x_actix_web::ProxyResponder,
    },
    x_outside_feature::proxy::feature::ProxyAppFeature,
};

use crate::auth::ticket::{
    authenticate::action::CheckAuthenticateTokenAction, logout::proxy::action::LogoutProxyAction,
};

use crate::{
    auth::ticket::{
        authenticate::data::CheckAuthenticateTokenError, logout::proxy::data::LogoutProxyError,
    },
    common::api::request::data::RequestInfo,
};

#[delete("")]
async fn service_logout(feature: Data<ProxyAppFeature>, request: HttpRequest) -> HttpResponse {
    async {
        let info = RequestInfo::from_request(&request);
        let logger = Arc::new(StdoutJsonLogger::with_request(info.clone()));

        let auth = CheckAuthenticateTokenAction::live(feature.as_infra())
            .with_logger(logger.clone())
            .check(&request)?;

        Ok::<_, AppError>(
            LogoutProxyAction::live(feature.as_infra())
                .with_logger(logger.clone())
                .logout(info, &request, auth)
                .await?,
        )
    }
    .await
    .respond_to()
}

enum AppError {
    CheckAuthenticateTokenError(CheckAuthenticateTokenError),
    LogoutError(LogoutProxyError),
}

impl From<CheckAuthenticateTokenError> for AppError {
    fn from(value: CheckAuthenticateTokenError) -> Self {
        Self::CheckAuthenticateTokenError(value)
    }
}

impl From<LogoutProxyError> for AppError {
    fn from(value: LogoutProxyError) -> Self {
        Self::LogoutError(value)
    }
}

impl ProxyResponder for AppError {
    fn respond_to(self) -> HttpResponse {
        match self {
            Self::CheckAuthenticateTokenError(err) => err.respond_to(),
            Self::LogoutError(err) => err.respond_to(),
        }
    }
}

impl ProxyResponder for LogoutProxyError {
    fn respond_to(self) -> HttpResponse {
        match self {
            Self::Invalid(err) => err.respond_to(),
            Self::ProxyError(err) => err.respond_to(),
        }
    }
}
