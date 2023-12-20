use std::sync::Arc;

use tonic::{Request, Response, Status};

use crate::auth::user::account::modify::y_protobuf::service::{
    modify_auth_user_account_pb_server::{ModifyAuthUserAccountPb, ModifyAuthUserAccountPbServer},
    ModifyAuthUserAccountErrorKindPb, ModifyAuthUserAccountRequestPb,
    ModifyAuthUserAccountResponsePb,
};

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::common::api::{
    feature::AsInfra, logger::detail::StdoutJsonLogger, response::x_tonic::ServiceResponder,
};

use crate::auth::{
    ticket::authorize::action::AuthorizeAction,
    user::account::modify::action::ModifyAuthUserAccountAction,
};

use crate::auth::user::account::modify::infra::{
    ModifyAuthUserAccountFields, ModifyAuthUserAccountFieldsExtract,
};

use crate::{
    auth::{
        ticket::authorize::data::AuthorizeError,
        user::{
            account::modify::data::{
                ModifyAuthUserAccountError, ModifyAuthUserAccountSuccess,
                ValidateModifyAuthUserAccountFieldsError,
            },
            login_id::kernel::data::LoginId,
        },
    },
    common::api::request::data::RequestInfo,
};

#[derive(Default)]
pub struct ServiceModifyUser;

impl ServiceModifyUser {
    pub fn server(&self) -> ModifyAuthUserAccountPbServer<Self> {
        ModifyAuthUserAccountPbServer::new(Self)
    }
}

#[async_trait::async_trait]
impl ModifyAuthUserAccountPb for ServiceModifyUser {
    async fn modify_user(
        &self,
        request: Request<ModifyAuthUserAccountRequestPb>,
    ) -> Result<Response<ModifyAuthUserAccountResponsePb>, Status> {
        async {
            let feature = AuthAppFeature::from_request(&request);
            let info = RequestInfo::from_metadata(request.metadata());
            let logger = Arc::new(StdoutJsonLogger::with_request(info));

            let (infra, _) = AuthorizeAction::live(feature.as_infra())
                .with_logger(logger.clone())
                .pick_authorized_infra(&feature, request.metadata())
                .await?;

            Ok::<_, AppError>(
                ModifyAuthUserAccountAction::live(infra)
                    .with_logger(logger.clone())
                    .modify(request.into_inner())
                    .await?,
            )
        }
        .await
        .respond_to()
    }
}

enum AppError {
    AuthorizeError(AuthorizeError),
    ModifyAuthUserAccountError(ModifyAuthUserAccountError),
}

impl From<AuthorizeError> for AppError {
    fn from(value: AuthorizeError) -> Self {
        Self::AuthorizeError(value)
    }
}

impl From<ModifyAuthUserAccountError> for AppError {
    fn from(value: ModifyAuthUserAccountError) -> Self {
        Self::ModifyAuthUserAccountError(value)
    }
}

impl ModifyAuthUserAccountFieldsExtract for ModifyAuthUserAccountRequestPb {
    fn convert(
        self,
    ) -> Result<ModifyAuthUserAccountFields, ValidateModifyAuthUserAccountFieldsError> {
        Ok(ModifyAuthUserAccountFields {
            login_id: LoginId::convert(self.login_id)
                .map_err(ValidateModifyAuthUserAccountFieldsError::InvalidLoginId)?,
            from: self
                .from
                .try_into()
                .map_err(ValidateModifyAuthUserAccountFieldsError::InvalidFrom)?,
            to: self
                .to
                .try_into()
                .map_err(ValidateModifyAuthUserAccountFieldsError::InvalidTo)?,
        })
    }
}

impl ServiceResponder<ModifyAuthUserAccountResponsePb> for ModifyAuthUserAccountSuccess {
    fn respond_to(self) -> Result<Response<ModifyAuthUserAccountResponsePb>, Status> {
        Ok(Response::new(ModifyAuthUserAccountResponsePb {
            success: true,
            ..Default::default()
        }))
    }
}

impl ServiceResponder<ModifyAuthUserAccountResponsePb> for AppError {
    fn respond_to(self) -> Result<Response<ModifyAuthUserAccountResponsePb>, Status> {
        match self {
            Self::AuthorizeError(err) => err.respond_to(),
            Self::ModifyAuthUserAccountError(err) => err.respond_to(),
        }
    }
}

impl ServiceResponder<ModifyAuthUserAccountResponsePb> for ModifyAuthUserAccountError {
    fn respond_to(self) -> Result<Response<ModifyAuthUserAccountResponsePb>, Status> {
        match self {
            Self::NotFound => Ok(Response::new(ModifyAuthUserAccountResponsePb {
                success: false,
                err: ModifyAuthUserAccountErrorKindPb::NotFound as i32,
                ..Default::default()
            })),
            Self::Conflict => Ok(Response::new(ModifyAuthUserAccountResponsePb {
                success: false,
                err: ModifyAuthUserAccountErrorKindPb::Conflict as i32,
                ..Default::default()
            })),
            Self::Invalid(_) => Ok(Response::new(ModifyAuthUserAccountResponsePb {
                success: false,
                err: ModifyAuthUserAccountErrorKindPb::Invalid as i32,
                ..Default::default()
            })),
            Self::RepositoryError(err) => err.respond_to(),
        }
    }
}
