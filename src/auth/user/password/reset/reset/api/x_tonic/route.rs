use std::sync::Arc;

use tonic::{Request, Response, Status};

use crate::auth::user::password::reset::reset::y_protobuf::service::{
    reset_password_pb_server::{ResetPasswordPb, ResetPasswordPbServer},
    ResetPasswordErrorKindPb, ResetPasswordRequestPb, ResetPasswordResponsePb,
};

use crate::{
    common::api::response::x_tonic::ServiceResponder,
    x_outside_feature::auth::feature::AuthAppFeature,
};

use crate::common::api::{feature::AsInfra, logger::detail::StdoutJsonLogger};

use crate::auth::{
    ticket::{encode::action::EncodeAuthTokenAction, issue::action::IssueAuthTicketAction},
    user::password::reset::reset::action::ResetPasswordAction,
};

use crate::auth::user::password::{
    kernel::infra::PlainPassword,
    reset::reset::infra::{ResetPasswordFields, ResetPasswordFieldsExtract},
};

use crate::{
    auth::{
        ticket::{
            encode::data::{EncodeAuthTokenError, EncodeAuthTokenSuccess},
            issue::data::IssueAuthTicketError,
        },
        user::password::reset::{
            kernel::data::ResetPasswordToken,
            reset::data::{
                DecodeResetTokenError, NotifyResetPasswordError, ResetPasswordError,
                ValidateResetPasswordFieldsError,
            },
        },
    },
    common::api::request::data::RequestInfo,
};

#[derive(Default)]
pub struct ServiceReset;

impl ServiceReset {
    pub fn server(&self) -> ResetPasswordPbServer<Self> {
        ResetPasswordPbServer::new(Self)
    }
}

#[async_trait::async_trait]
impl ResetPasswordPb for ServiceReset {
    async fn reset(
        &self,
        request: Request<ResetPasswordRequestPb>,
    ) -> Result<Response<ResetPasswordResponsePb>, Status> {
        async {
            let feature = AuthAppFeature::from_request(&request);
            let info = RequestInfo::from_metadata(request.metadata());
            let logger = Arc::new(StdoutJsonLogger::with_request(info));

            let auth = ResetPasswordAction::live(feature.as_infra())
                .with_logger(logger.clone())
                .reset(request.into_inner())
                .await?;

            let ticket = IssueAuthTicketAction::live(feature.as_infra())
                .with_logger(logger.clone())
                .issue(auth)
                .await?;

            Ok::<_, AppError>(
                EncodeAuthTokenAction::live(feature.as_infra())
                    .with_logger(logger)
                    .encode(ticket)
                    .await?,
            )
        }
        .await
        .respond_to()
    }
}

impl ResetPasswordFieldsExtract for ResetPasswordRequestPb {
    fn convert(self) -> Result<ResetPasswordFields, ValidateResetPasswordFieldsError> {
        Ok(ResetPasswordFields {
            reset_token: ResetPasswordToken::convert(self.reset_token)
                .map_err(ValidateResetPasswordFieldsError::InvalidResetToken)?,
            new_password: PlainPassword::convert(self.new_password)
                .map_err(ValidateResetPasswordFieldsError::InvalidNewPassword)?,
        })
    }
}

impl ServiceResponder<ResetPasswordResponsePb> for EncodeAuthTokenSuccess {
    fn respond_to(self) -> Result<Response<ResetPasswordResponsePb>, Status> {
        let (token, granted) = self.extract();
        Ok(Response::new(ResetPasswordResponsePb {
            success: true,
            token: Some(token.into()),
            granted: Some(granted.into()),
            ..Default::default()
        }))
    }
}

enum AppError {
    ResetPasswordError(ResetPasswordError),
    IssueAuthTicketError(IssueAuthTicketError),
    EncodeAuthTokenError(EncodeAuthTokenError),
}

impl From<ResetPasswordError> for AppError {
    fn from(value: ResetPasswordError) -> Self {
        Self::ResetPasswordError(value)
    }
}

impl From<IssueAuthTicketError> for AppError {
    fn from(value: IssueAuthTicketError) -> Self {
        Self::IssueAuthTicketError(value)
    }
}

impl From<EncodeAuthTokenError> for AppError {
    fn from(value: EncodeAuthTokenError) -> Self {
        Self::EncodeAuthTokenError(value)
    }
}

impl ServiceResponder<ResetPasswordResponsePb> for AppError {
    fn respond_to(self) -> Result<Response<ResetPasswordResponsePb>, Status> {
        match self {
            Self::ResetPasswordError(err) => err.respond_to(),
            Self::IssueAuthTicketError(err) => err.respond_to(),
            Self::EncodeAuthTokenError(err) => err.respond_to(),
        }
    }
}

impl ServiceResponder<ResetPasswordResponsePb> for ResetPasswordError {
    fn respond_to(self) -> Result<Response<ResetPasswordResponsePb>, Status> {
        match self {
            Self::Invalid(_) => Ok(Response::new(ResetPasswordResponsePb {
                success: false,
                err: ResetPasswordErrorKindPb::InvalidReset as i32,
                ..Default::default()
            })),
            Self::NotFound => Ok(Response::new(ResetPasswordResponsePb {
                success: false,
                err: ResetPasswordErrorKindPb::InvalidReset as i32,
                ..Default::default()
            })),
            Self::ResetTokenExpired => Ok(Response::new(ResetPasswordResponsePb {
                success: false,
                err: ResetPasswordErrorKindPb::InvalidReset as i32,
                ..Default::default()
            })),
            Self::AlreadyReset => Ok(Response::new(ResetPasswordResponsePb {
                success: false,
                err: ResetPasswordErrorKindPb::AlreadyReset as i32,
                ..Default::default()
            })),
            Self::RepositoryError(err) => err.respond_to(),
            Self::PasswordHashError(err) => err.respond_to(),
            Self::DecodeError(err) => err.respond_to(),
            Self::NotifyError(err) => err.respond_to(),
        }
    }
}

impl<T> ServiceResponder<T> for DecodeResetTokenError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        Err(Status::unauthenticated("failed to decode reset token"))
    }
}

impl<T> ServiceResponder<T> for NotifyResetPasswordError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        match self {
            Self::NotificationError(_) => Err(Status::internal("notify reset password error")),
        }
    }
}
