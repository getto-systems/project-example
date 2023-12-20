use std::sync::Arc;

use tonic::{Request, Response, Status};

use crate::{
    auth::{
        ticket::authorize::{action::AuthorizeAction, data::AuthorizeError},
        user::{
            account::unregister::{
                action::UnregisterAuthUserAccountAction,
                data::{UnregisterAuthUserAccountError, UnregisterAuthUserAccountSuccess},
                infra::{UnregisterAuthUserAccountFields, UnregisterAuthUserAccountFieldsExtract},
                y_protobuf::service::{
                    unregister_auth_user_account_pb_server::{
                        UnregisterAuthUserAccountPb, UnregisterAuthUserAccountPbServer,
                    },
                    UnregisterAuthUserAccountRequestPb, UnregisterAuthUserAccountResponsePb,
                },
            },
            login_id::kernel::data::{LoginId, ValidateLoginIdError},
        },
    },
    common::api::feature::AsInfra,
};

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::common::api::{logger::detail::StdoutJsonLogger, response::x_tonic::ServiceResponder};

use crate::common::api::request::data::RequestInfo;

#[derive(Default)]
pub struct ServiceUnregisterUser;

impl ServiceUnregisterUser {
    pub fn server(&self) -> UnregisterAuthUserAccountPbServer<Self> {
        UnregisterAuthUserAccountPbServer::new(Self)
    }
}

#[async_trait::async_trait]
impl UnregisterAuthUserAccountPb for ServiceUnregisterUser {
    async fn unregister_user(
        &self,
        request: Request<UnregisterAuthUserAccountRequestPb>,
    ) -> Result<Response<UnregisterAuthUserAccountResponsePb>, Status> {
        async {
            let feature = AuthAppFeature::from_request(&request);
            let info = RequestInfo::from_metadata(request.metadata());
            let logger = Arc::new(StdoutJsonLogger::with_request(info));

            let (infra, _) = AuthorizeAction::live(feature.as_infra())
                .with_logger(logger.clone())
                .pick_authorized_infra(&feature, request.metadata())
                .await?;

            Ok::<_, AppError>(
                UnregisterAuthUserAccountAction::live(infra)
                    .with_logger(logger.clone())
                    .unregister(request.into_inner())
                    .await?,
            )
        }
        .await
        .respond_to()
    }
}

impl UnregisterAuthUserAccountFieldsExtract for UnregisterAuthUserAccountRequestPb {
    fn convert(self) -> Result<UnregisterAuthUserAccountFields, ValidateLoginIdError> {
        Ok(UnregisterAuthUserAccountFields {
            login_id: LoginId::convert(self.login_id)?,
        })
    }
}

impl ServiceResponder<UnregisterAuthUserAccountResponsePb> for UnregisterAuthUserAccountSuccess {
    fn respond_to(self) -> Result<Response<UnregisterAuthUserAccountResponsePb>, Status> {
        Ok(Response::new(UnregisterAuthUserAccountResponsePb {
            success: true,
            ..Default::default()
        }))
    }
}

enum AppError {
    AuthorizeError(AuthorizeError),
    UnregisterAuthUserAccountError(UnregisterAuthUserAccountError),
}

impl From<AuthorizeError> for AppError {
    fn from(value: AuthorizeError) -> Self {
        Self::AuthorizeError(value)
    }
}

impl From<UnregisterAuthUserAccountError> for AppError {
    fn from(value: UnregisterAuthUserAccountError) -> Self {
        Self::UnregisterAuthUserAccountError(value)
    }
}

impl ServiceResponder<UnregisterAuthUserAccountResponsePb> for AppError {
    fn respond_to(self) -> Result<Response<UnregisterAuthUserAccountResponsePb>, Status> {
        match self {
            Self::AuthorizeError(err) => err.respond_to(),
            Self::UnregisterAuthUserAccountError(err) => err.respond_to(),
        }
    }
}

impl ServiceResponder<UnregisterAuthUserAccountResponsePb> for UnregisterAuthUserAccountError {
    fn respond_to(self) -> Result<Response<UnregisterAuthUserAccountResponsePb>, Status> {
        match self {
            Self::Invalid(_) => Ok(Response::new(UnregisterAuthUserAccountResponsePb {
                success: false,
                ..Default::default()
            })),
            Self::NotFound => Ok(Response::new(UnregisterAuthUserAccountResponsePb {
                success: false,
                ..Default::default()
            })),
            Self::RepositoryError(err) => err.respond_to(),
        }
    }
}
