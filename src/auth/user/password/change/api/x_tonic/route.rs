use std::sync::Arc;

use tonic::{Request, Response, Status};

use crate::auth::user::password::change::y_protobuf::service::{
    change_password_pb_server::{ChangePasswordPb, ChangePasswordPbServer},
    overwrite_password_pb_server::{OverwritePasswordPb, OverwritePasswordPbServer},
    ChangePasswordRequestPb, ChangePasswordResponsePb, OverwritePasswordRequestPb,
    OverwritePasswordResponsePb,
};

use crate::{
    common::api::response::x_tonic::ServiceResponder,
    x_outside_feature::auth::feature::AuthAppFeature,
};

use crate::common::api::{feature::AsInfra, logger::detail::StdoutJsonLogger};

use crate::auth::{
    ticket::authorize::action::AuthorizeAction,
    user::password::change::action::{ChangePasswordAction, OverwritePasswordAction},
};

use crate::auth::user::password::{
    change::infra::{
        ChangePasswordFields, ChangePasswordFieldsExtract, OverwritePasswordFields,
        OverwritePasswordFieldsExtract,
    },
    kernel::infra::PlainPassword,
};

use crate::{
    auth::user::{
        login_id::kernel::data::LoginId,
        password::change::data::{
            ChangePasswordError, ChangePasswordSuccess, OverwritePasswordError,
            OverwritePasswordSuccess, ValidateChangePasswordFieldsError,
            ValidateOverwritePasswordFieldsError,
        },
    },
    common::api::request::data::RequestInfo,
};

#[derive(Default)]
pub struct ServiceChangePassword;

impl ServiceChangePassword {
    pub fn server(&self) -> ChangePasswordPbServer<Self> {
        ChangePasswordPbServer::new(Self)
    }
}

#[async_trait::async_trait]
impl ChangePasswordPb for ServiceChangePassword {
    async fn change_password(
        &self,
        request: Request<ChangePasswordRequestPb>,
    ) -> Result<Response<ChangePasswordResponsePb>, Status> {
        async {
            let feature = AuthAppFeature::from_request(&request);
            let info = RequestInfo::from_metadata(request.metadata());
            let logger = Arc::new(StdoutJsonLogger::with_request(info));

            let (infra, auth) = AuthorizeAction::live(feature.as_infra())
                .with_logger(logger.clone())
                .pick_authorized_infra(&feature, request.metadata())
                .await?;

            ChangePasswordAction::live(infra)
                .with_logger(logger)
                .change(auth.extract().user_id, request.into_inner())
                .await
        }
        .await
        .respond_to()
    }
}

impl ChangePasswordFieldsExtract for ChangePasswordRequestPb {
    fn convert(self) -> Result<ChangePasswordFields, ValidateChangePasswordFieldsError> {
        Ok(ChangePasswordFields {
            current_password: PlainPassword::convert(self.current_password)
                .map_err(ValidateChangePasswordFieldsError::InvalidCurrentPassword)?,
            new_password: PlainPassword::convert(self.new_password)
                .map_err(ValidateChangePasswordFieldsError::InvalidNewPassword)?,
        })
    }
}

impl ServiceResponder<ChangePasswordResponsePb> for ChangePasswordSuccess {
    fn respond_to(self) -> Result<Response<ChangePasswordResponsePb>, Status> {
        Ok(Response::new(ChangePasswordResponsePb { success: true }))
    }
}

impl ServiceResponder<ChangePasswordResponsePb> for ChangePasswordError {
    fn respond_to(self) -> Result<Response<ChangePasswordResponsePb>, Status> {
        match self {
            Self::Invalid(err) => err.respond_to(),
            Self::NotFound => Ok(Response::new(ChangePasswordResponsePb { success: false })),
            Self::PasswordNotMatched => {
                Ok(Response::new(ChangePasswordResponsePb { success: false }))
            }
            Self::PasswordHashError(err) => err.respond_to(),
            Self::RepositoryError(err) => err.respond_to(),
            Self::AuthorizeError(err) => err.respond_to(),
        }
    }
}

impl ServiceResponder<ChangePasswordResponsePb> for ValidateChangePasswordFieldsError {
    fn respond_to(self) -> Result<Response<ChangePasswordResponsePb>, Status> {
        Ok(Response::new(ChangePasswordResponsePb { success: false }))
    }
}

#[derive(Default)]
pub struct ServiceOverwritePassword;

impl ServiceOverwritePassword {
    pub fn server(&self) -> OverwritePasswordPbServer<Self> {
        OverwritePasswordPbServer::new(Self)
    }
}

#[async_trait::async_trait]
impl OverwritePasswordPb for ServiceOverwritePassword {
    async fn overwrite_password(
        &self,
        request: Request<OverwritePasswordRequestPb>,
    ) -> Result<Response<OverwritePasswordResponsePb>, Status> {
        async {
            let feature = AuthAppFeature::from_request(&request);
            let info = RequestInfo::from_metadata(request.metadata());
            let logger = Arc::new(StdoutJsonLogger::with_request(info));

            let (infra, _auth) = AuthorizeAction::live(feature.as_infra())
                .with_logger(logger.clone())
                .pick_authorized_infra(&feature, request.metadata())
                .await?;

            OverwritePasswordAction::live(infra)
                .with_logger(logger)
                .overwrite(request.into_inner())
                .await
        }
        .await
        .respond_to()
    }
}

impl OverwritePasswordFieldsExtract for OverwritePasswordRequestPb {
    fn convert(self) -> Result<OverwritePasswordFields, ValidateOverwritePasswordFieldsError> {
        Ok(OverwritePasswordFields {
            login_id: LoginId::convert(self.login_id)
                .map_err(ValidateOverwritePasswordFieldsError::InvalidLoginId)?,
            new_password: PlainPassword::convert(self.new_password)
                .map_err(ValidateOverwritePasswordFieldsError::InvalidNewPassword)?,
        })
    }
}

impl ServiceResponder<OverwritePasswordResponsePb> for OverwritePasswordSuccess {
    fn respond_to(self) -> Result<Response<OverwritePasswordResponsePb>, Status> {
        Ok(Response::new(OverwritePasswordResponsePb { success: true }))
    }
}

impl ServiceResponder<OverwritePasswordResponsePb> for OverwritePasswordError {
    fn respond_to(self) -> Result<Response<OverwritePasswordResponsePb>, Status> {
        match self {
            Self::Invalid(_) => Ok(Response::new(OverwritePasswordResponsePb {
                success: false,
            })),
            Self::NotFound => Ok(Response::new(OverwritePasswordResponsePb {
                success: false,
            })),
            Self::PasswordHashError(err) => err.respond_to(),
            Self::RepositoryError(err) => err.respond_to(),
            Self::AuthorizeError(err) => err.respond_to(),
        }
    }
}
