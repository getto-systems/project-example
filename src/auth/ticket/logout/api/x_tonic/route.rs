use std::sync::Arc;

use tonic::{Request, Response, Status};

use crate::auth::ticket::logout::y_protobuf::service::{
    logout_pb_server::{LogoutPb, LogoutPbServer},
    LogoutRequestPb, LogoutResponsePb,
};

use crate::{
    common::api::{
        feature::AsInfra, logger::detail::StdoutJsonLogger, response::x_tonic::ServiceResponder,
    },
    x_outside_feature::auth::feature::AuthAppFeature,
};

use crate::auth::ticket::{
    authenticate::action::CheckAuthenticateTokenAction, logout::action::LogoutAction,
};

use crate::{
    auth::ticket::{
        authenticate::data::CheckAuthenticateTokenError,
        logout::data::{LogoutError, LogoutSuccess},
    },
    common::api::request::data::RequestInfo,
};

#[derive(Default)]
pub struct ServiceLogout;

impl ServiceLogout {
    pub fn server(&self) -> LogoutPbServer<ServiceLogout> {
        LogoutPbServer::new(ServiceLogout)
    }
}

#[async_trait::async_trait]
impl LogoutPb for ServiceLogout {
    async fn logout(
        &self,
        request: Request<LogoutRequestPb>,
    ) -> Result<Response<LogoutResponsePb>, Status> {
        async {
            let feature = AuthAppFeature::from_request(&request);
            let info = RequestInfo::from_metadata(request.metadata());
            let logger = Arc::new(StdoutJsonLogger::with_request(info));

            let auth = CheckAuthenticateTokenAction::live(feature.as_infra())
                .with_logger(logger.clone())
                .check(request.metadata())?;

            Ok::<_, AppError>(
                LogoutAction::live(feature.as_infra())
                    .with_logger(logger)
                    .logout(auth)
                    .await?,
            )
        }
        .await
        .respond_to()
    }
}

impl ServiceResponder<LogoutResponsePb> for LogoutSuccess {
    fn respond_to(self) -> Result<Response<LogoutResponsePb>, Status> {
        Ok(Response::new(LogoutResponsePb {}))
    }
}

enum AppError {
    CheckAuthenticateTokenError(CheckAuthenticateTokenError),
    LogoutError(LogoutError),
}

impl From<CheckAuthenticateTokenError> for AppError {
    fn from(value: CheckAuthenticateTokenError) -> Self {
        Self::CheckAuthenticateTokenError(value)
    }
}

impl From<LogoutError> for AppError {
    fn from(value: LogoutError) -> Self {
        Self::LogoutError(value)
    }
}

impl<T> ServiceResponder<T> for AppError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        match self {
            Self::CheckAuthenticateTokenError(err) => err.respond_to(),
            Self::LogoutError(err) => err.respond_to(),
        }
    }
}

impl<T> ServiceResponder<T> for LogoutError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        match self {
            Self::RepositoryError(err) => err.respond_to(),
        }
    }
}
