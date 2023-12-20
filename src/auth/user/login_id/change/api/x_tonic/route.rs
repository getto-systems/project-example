use std::sync::Arc;

use tonic::{Request, Response, Status};

use crate::auth::user::login_id::change::y_protobuf::service::{
    overwrite_login_id_pb_server::{OverwriteLoginIdPb, OverwriteLoginIdPbServer},
    OverwriteLoginIdErrorKindPb, OverwriteLoginIdRequestPb, OverwriteLoginIdResponsePb,
};

use crate::{
    common::api::response::x_tonic::ServiceResponder,
    x_outside_feature::auth::feature::AuthAppFeature,
};

use crate::common::api::{feature::AsInfra, logger::detail::StdoutJsonLogger};

use crate::auth::{
    ticket::authorize::action::AuthorizeAction,
    user::login_id::change::action::OverwriteLoginIdAction,
};

use crate::auth::user::login_id::change::infra::{
    OverwriteLoginIdFields, OverwriteLoginIdFieldsExtract,
};

use crate::{
    auth::{
        ticket::authorize::data::AuthorizeError,
        user::login_id::{
            change::data::{
                OverwriteLoginIdError, OverwriteLoginIdSuccess, ValidateOverwriteLoginIdFieldsError,
            },
            kernel::data::LoginId,
        },
    },
    common::api::request::data::RequestInfo,
};

#[derive(Default)]
pub struct ServiceOverwriteLoginId;

impl ServiceOverwriteLoginId {
    pub fn server(&self) -> OverwriteLoginIdPbServer<Self> {
        OverwriteLoginIdPbServer::new(Self)
    }
}

#[async_trait::async_trait]
impl OverwriteLoginIdPb for ServiceOverwriteLoginId {
    async fn overwrite_login_id(
        &self,
        request: Request<OverwriteLoginIdRequestPb>,
    ) -> Result<Response<OverwriteLoginIdResponsePb>, Status> {
        async {
            let feature = AuthAppFeature::from_request(&request);
            let info = RequestInfo::from_metadata(request.metadata());
            let logger = Arc::new(StdoutJsonLogger::with_request(info));

            let (infra, _) = AuthorizeAction::live(feature.as_infra())
                .with_logger(logger.clone())
                .pick_authorized_infra(&feature, request.metadata())
                .await?;

            Ok::<_, AppError>(
                OverwriteLoginIdAction::live(infra)
                    .with_logger(logger.clone())
                    .overwrite(request.into_inner())
                    .await?,
            )
        }
        .await
        .respond_to()
    }
}

enum AppError {
    AuthorizeError(AuthorizeError),
    OverwriteLoginIdError(OverwriteLoginIdError),
}

impl From<AuthorizeError> for AppError {
    fn from(value: AuthorizeError) -> Self {
        Self::AuthorizeError(value)
    }
}

impl From<OverwriteLoginIdError> for AppError {
    fn from(value: OverwriteLoginIdError) -> Self {
        Self::OverwriteLoginIdError(value)
    }
}

impl OverwriteLoginIdFieldsExtract for OverwriteLoginIdRequestPb {
    fn convert(self) -> Result<OverwriteLoginIdFields, ValidateOverwriteLoginIdFieldsError> {
        Ok(OverwriteLoginIdFields {
            login_id: LoginId::convert(self.login_id)
                .map_err(ValidateOverwriteLoginIdFieldsError::InvalidCurrentLoginId)?,
            new_login_id: LoginId::convert(self.new_login_id)
                .map_err(ValidateOverwriteLoginIdFieldsError::InvalidNewLoginId)?,
        })
    }
}

impl ServiceResponder<OverwriteLoginIdResponsePb> for OverwriteLoginIdSuccess {
    fn respond_to(self) -> Result<Response<OverwriteLoginIdResponsePb>, Status> {
        Ok(Response::new(OverwriteLoginIdResponsePb {
            success: true,
            ..Default::default()
        }))
    }
}

impl ServiceResponder<OverwriteLoginIdResponsePb> for AppError {
    fn respond_to(self) -> Result<Response<OverwriteLoginIdResponsePb>, Status> {
        match self {
            Self::AuthorizeError(err) => err.respond_to(),
            Self::OverwriteLoginIdError(err) => err.respond_to(),
        }
    }
}

impl ServiceResponder<OverwriteLoginIdResponsePb> for OverwriteLoginIdError {
    fn respond_to(self) -> Result<Response<OverwriteLoginIdResponsePb>, Status> {
        match self {
            Self::Invalid(_) => Ok(Response::new(OverwriteLoginIdResponsePb {
                success: false,
                err: OverwriteLoginIdErrorKindPb::Invalid as i32,
            })),
            Self::NotFound => Ok(Response::new(OverwriteLoginIdResponsePb {
                success: false,
                err: OverwriteLoginIdErrorKindPb::NotFound as i32,
            })),
            Self::AlreadyRegistered => Ok(Response::new(OverwriteLoginIdResponsePb {
                success: false,
                err: OverwriteLoginIdErrorKindPb::AlreadyRegistered as i32,
            })),
            Self::RepositoryError(err) => err.respond_to(),
        }
    }
}
