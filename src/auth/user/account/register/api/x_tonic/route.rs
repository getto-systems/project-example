use std::sync::Arc;

use tonic::{Request, Response, Status};

use crate::auth::user::account::register::y_protobuf::service::{
    register_auth_user_account_pb_server::{
        RegisterAuthUserAccountPb, RegisterAuthUserAccountPbServer,
    },
    RegisterAuthUserAccountErrorKindPb, RegisterAuthUserAccountRequestPb,
    RegisterAuthUserAccountResponsePb,
};

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::common::api::{
    feature::AsInfra, logger::detail::StdoutJsonLogger, response::x_tonic::ServiceResponder,
};

use crate::auth::{
    ticket::authorize::action::AuthorizeAction,
    user::account::register::action::RegisterAuthUserAccountAction,
};

use crate::auth::user::account::register::infra::RegisterAuthUserAccountFieldsExtract;

use crate::{
    auth::{
        ticket::authorize::data::AuthorizeError,
        user::account::{
            kernel::data::{AuthUserAccount, ValidateAuthUserAccountError},
            register::data::{RegisterAuthUserAccountError, RegisterAuthUserAccountSuccess},
        },
    },
    common::api::request::data::RequestInfo,
};

#[derive(Default)]
pub struct ServiceRegisterUser;

impl ServiceRegisterUser {
    pub fn server(&self) -> RegisterAuthUserAccountPbServer<Self> {
        RegisterAuthUserAccountPbServer::new(Self)
    }
}

#[async_trait::async_trait]
impl RegisterAuthUserAccountPb for ServiceRegisterUser {
    async fn register_user(
        &self,
        request: Request<RegisterAuthUserAccountRequestPb>,
    ) -> Result<Response<RegisterAuthUserAccountResponsePb>, Status> {
        async {
            let feature = AuthAppFeature::from_request(&request);
            let info = RequestInfo::from_metadata(request.metadata());
            let logger = Arc::new(StdoutJsonLogger::with_request(info));

            let (infra, _) = AuthorizeAction::live(feature.as_infra())
                .with_logger(logger.clone())
                .pick_authorized_infra(&feature, request.metadata())
                .await?;

            Ok::<_, AppError>(
                RegisterAuthUserAccountAction::live(infra)
                    .with_logger(logger.clone())
                    .register(request.into_inner())
                    .await?,
            )
        }
        .await
        .respond_to()
    }
}

enum AppError {
    AuthorizeError(AuthorizeError),
    RegisterAuthUserAccountError(RegisterAuthUserAccountError),
}

impl From<AuthorizeError> for AppError {
    fn from(value: AuthorizeError) -> Self {
        Self::AuthorizeError(value)
    }
}

impl From<RegisterAuthUserAccountError> for AppError {
    fn from(value: RegisterAuthUserAccountError) -> Self {
        Self::RegisterAuthUserAccountError(value)
    }
}

impl RegisterAuthUserAccountFieldsExtract for RegisterAuthUserAccountRequestPb {
    fn convert(self) -> Result<AuthUserAccount, ValidateAuthUserAccountError> {
        self.data.try_into()
    }
}

impl ServiceResponder<RegisterAuthUserAccountResponsePb> for RegisterAuthUserAccountSuccess {
    fn respond_to(self) -> Result<Response<RegisterAuthUserAccountResponsePb>, Status> {
        Ok(Response::new(RegisterAuthUserAccountResponsePb {
            success: true,
            ..Default::default()
        }))
    }
}

impl ServiceResponder<RegisterAuthUserAccountResponsePb> for AppError {
    fn respond_to(self) -> Result<Response<RegisterAuthUserAccountResponsePb>, Status> {
        match self {
            Self::AuthorizeError(err) => err.respond_to(),
            Self::RegisterAuthUserAccountError(err) => err.respond_to(),
        }
    }
}

impl ServiceResponder<RegisterAuthUserAccountResponsePb> for RegisterAuthUserAccountError {
    fn respond_to(self) -> Result<Response<RegisterAuthUserAccountResponsePb>, Status> {
        match self {
            Self::LoginIdAlreadyRegistered => {
                Ok(Response::new(RegisterAuthUserAccountResponsePb {
                    success: false,
                    err: RegisterAuthUserAccountErrorKindPb::LoginIdAlreadyRegistered as i32,
                    ..Default::default()
                }))
            }
            Self::Invalid(_) => Ok(Response::new(RegisterAuthUserAccountResponsePb {
                success: false,
                err: RegisterAuthUserAccountErrorKindPb::Invalid as i32,
                ..Default::default()
            })),
            Self::RepositoryError(err) => err.respond_to(),
        }
    }
}
