use std::sync::Arc;

use tonic::{Request, Response, Status};

use crate::auth::ticket::authenticate::y_protobuf::service::{
    authenticate_with_token_pb_server::{AuthenticateWithTokenPb, AuthenticateWithTokenPbServer},
    AuthenticateWithTokenRequestPb, AuthenticateWithTokenResponsePb,
};

use crate::{
    common::api::{
        feature::AsInfra, logger::detail::StdoutJsonLogger, response::x_tonic::ServiceResponder,
    },
    x_outside_feature::auth::feature::AuthAppFeature,
};

use crate::auth::ticket::{
    authenticate::action::CheckAuthenticateTokenAction, encode::action::EncodeAuthTokenAction,
};

use crate::{
    auth::ticket::{
        authenticate::data::CheckAuthenticateTokenError,
        encode::data::{EncodeAuthTokenError, EncodeAuthTokenSuccess},
    },
    common::api::request::data::RequestInfo,
};

#[derive(Default)]
pub struct ServiceAuthenticateWithToken;

impl ServiceAuthenticateWithToken {
    pub fn server(&self) -> AuthenticateWithTokenPbServer<ServiceAuthenticateWithToken> {
        AuthenticateWithTokenPbServer::new(ServiceAuthenticateWithToken)
    }
}

#[async_trait::async_trait]
impl AuthenticateWithTokenPb for ServiceAuthenticateWithToken {
    async fn call(
        &self,
        request: Request<AuthenticateWithTokenRequestPb>,
    ) -> Result<Response<AuthenticateWithTokenResponsePb>, Status> {
        async {
            let feature = AuthAppFeature::from_request(&request);
            let info = RequestInfo::from_metadata(request.metadata());
            let logger = Arc::new(StdoutJsonLogger::with_request(info));

            let auth = CheckAuthenticateTokenAction::live(feature.as_infra())
                .with_logger(logger.clone())
                .check(request.metadata())?;

            Ok::<_, AppError>(
                EncodeAuthTokenAction::live(feature.as_infra())
                    .with_logger(logger)
                    .encode(auth.extract())
                    .await?,
            )
        }
        .await
        .respond_to()
    }
}

enum AppError {
    CheckAuthenticateTokenError(CheckAuthenticateTokenError),
    EncodeAuthTokenError(EncodeAuthTokenError),
}

impl From<CheckAuthenticateTokenError> for AppError {
    fn from(value: CheckAuthenticateTokenError) -> Self {
        Self::CheckAuthenticateTokenError(value)
    }
}

impl From<EncodeAuthTokenError> for AppError {
    fn from(value: EncodeAuthTokenError) -> Self {
        Self::EncodeAuthTokenError(value)
    }
}

impl ServiceResponder<AuthenticateWithTokenResponsePb> for EncodeAuthTokenSuccess {
    fn respond_to(self) -> Result<Response<AuthenticateWithTokenResponsePb>, Status> {
        let (token, granted) = self.extract();
        Ok(Response::new(AuthenticateWithTokenResponsePb {
            token: Some(token.into()),
            granted: Some(granted.into()),
        }))
    }
}

impl<T> ServiceResponder<T> for AppError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        match self {
            Self::CheckAuthenticateTokenError(err) => err.respond_to(),
            Self::EncodeAuthTokenError(err) => err.respond_to(),
        }
    }
}

impl<T> ServiceResponder<T> for CheckAuthenticateTokenError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        match self {
            Self::Invalid(err) => err.respond_to(),
            Self::DecodeError(err) => err.respond_to(),
        }
    }
}
