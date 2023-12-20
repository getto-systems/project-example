use std::sync::Arc;

use tonic::{Request, Response, Status};

use crate::auth::user::password::reset::token_destination::change::y_protobuf::service::change_reset_token_destination_pb_server::ChangeResetTokenDestinationPbServer;
use crate::auth::user::password::reset::token_destination::change::y_protobuf::service::{
    change_reset_token_destination_pb_server::ChangeResetTokenDestinationPb,
    ChangeResetTokenDestinationErrorKindPb, ChangeResetTokenDestinationRequestPb,
    ChangeResetTokenDestinationResponsePb,
};

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::common::api::{
    feature::AsInfra, logger::detail::StdoutJsonLogger, response::x_tonic::ServiceResponder,
};

use crate::auth::{
    ticket::authorize::action::AuthorizeAction,
    user::password::reset::token_destination::change::action::ChangeResetTokenDestinationAction,
};

use crate::auth::user::password::reset::token_destination::change::infra::{
    ChangeResetTokenDestinationFields, ChangeResetTokenDestinationFieldsExtract,
};

use crate::{
    auth::{
        ticket::authorize::data::AuthorizeError,
        user::{
            login_id::kernel::data::LoginId,
            password::reset::token_destination::change::data::{
                ChangeResetTokenDestinationError, ChangeResetTokenDestinationSuccess,
                ValidateChangeResetTokenDestinationFieldsError,
            },
        },
    },
    common::api::request::data::RequestInfo,
};

#[derive(Default)]
pub struct ServiceChangeDestination;

impl ServiceChangeDestination {
    pub fn server(&self) -> ChangeResetTokenDestinationPbServer<Self> {
        ChangeResetTokenDestinationPbServer::new(Self)
    }
}

#[async_trait::async_trait]
impl ChangeResetTokenDestinationPb for ServiceChangeDestination {
    async fn change_destination(
        &self,
        request: Request<ChangeResetTokenDestinationRequestPb>,
    ) -> Result<Response<ChangeResetTokenDestinationResponsePb>, Status> {
        async {
            let feature = AuthAppFeature::from_request(&request);
            let info = RequestInfo::from_metadata(request.metadata());
            let logger = Arc::new(StdoutJsonLogger::with_request(info));

            let (infra, _) = AuthorizeAction::live(feature.as_infra())
                .with_logger(logger.clone())
                .pick_authorized_infra(&feature, request.metadata())
                .await?;

            Ok::<_, AppError>(
                ChangeResetTokenDestinationAction::live(infra)
                    .with_logger(logger.clone())
                    .change(request.into_inner())
                    .await?,
            )
        }
        .await
        .respond_to()
    }
}

impl ChangeResetTokenDestinationFieldsExtract for ChangeResetTokenDestinationRequestPb {
    fn convert(
        self,
    ) -> Result<ChangeResetTokenDestinationFields, ValidateChangeResetTokenDestinationFieldsError>
    {
        Ok(ChangeResetTokenDestinationFields {
            login_id: LoginId::convert(self.login_id)
                .map_err(ValidateChangeResetTokenDestinationFieldsError::InvalidLoginId)?,
            from: self
                .from
                .try_into()
                .map_err(ValidateChangeResetTokenDestinationFieldsError::InvalidFrom)?,
            to: self
                .to
                .try_into()
                .map_err(ValidateChangeResetTokenDestinationFieldsError::InvalidTo)?,
        })
    }
}

impl ServiceResponder<ChangeResetTokenDestinationResponsePb>
    for ChangeResetTokenDestinationSuccess
{
    fn respond_to(self) -> Result<Response<ChangeResetTokenDestinationResponsePb>, Status> {
        Ok(Response::new(ChangeResetTokenDestinationResponsePb {
            success: true,
            ..Default::default()
        }))
    }
}

enum AppError {
    AuthorizeError(AuthorizeError),
    ChangeResetTokenDestinationError(ChangeResetTokenDestinationError),
}

impl From<AuthorizeError> for AppError {
    fn from(value: AuthorizeError) -> Self {
        Self::AuthorizeError(value)
    }
}

impl From<ChangeResetTokenDestinationError> for AppError {
    fn from(value: ChangeResetTokenDestinationError) -> Self {
        Self::ChangeResetTokenDestinationError(value)
    }
}

impl ServiceResponder<ChangeResetTokenDestinationResponsePb> for AppError {
    fn respond_to(self) -> Result<Response<ChangeResetTokenDestinationResponsePb>, Status> {
        match self {
            Self::AuthorizeError(err) => err.respond_to(),
            Self::ChangeResetTokenDestinationError(err) => err.respond_to(),
        }
    }
}

impl ServiceResponder<ChangeResetTokenDestinationResponsePb> for ChangeResetTokenDestinationError {
    fn respond_to(self) -> Result<Response<ChangeResetTokenDestinationResponsePb>, Status> {
        match self {
            Self::NotFound => Ok(Response::new(ChangeResetTokenDestinationResponsePb {
                success: false,
                err: ChangeResetTokenDestinationErrorKindPb::NotFound as i32,
                ..Default::default()
            })),
            Self::Conflict => Ok(Response::new(ChangeResetTokenDestinationResponsePb {
                success: false,
                err: ChangeResetTokenDestinationErrorKindPb::Conflict as i32,
                ..Default::default()
            })),
            Self::Invalid(_) => Ok(Response::new(ChangeResetTokenDestinationResponsePb {
                success: false,
                err: ChangeResetTokenDestinationErrorKindPb::Invalid as i32,
                ..Default::default()
            })),
            Self::RepositoryError(err) => err.respond_to(),
        }
    }
}
