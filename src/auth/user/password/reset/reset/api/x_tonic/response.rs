use tonic::{Response, Status};

use crate::common::api::response::tonic::ServiceResponder;

use crate::auth::user::password::reset::reset::y_protobuf::service::{
    ResetPasswordErrorKindPb, ResetPasswordResponsePb,
};

use crate::auth::user::password::reset::reset::action::{ResetPasswordEvent, ResetPasswordState};

use crate::auth::ticket::encode::method::EncodeAuthTokenEvent;

use crate::auth::{
    ticket::kernel::data::{AuthPermissionGranted, AuthToken},
    user::password::reset::reset::data::{DecodeResetTokenError, NotifyResetPasswordError},
};

impl ServiceResponder<ResetPasswordResponsePb> for ResetPasswordState {
    fn respond_to(self) -> Result<Response<ResetPasswordResponsePb>, Status> {
        match self {
            Self::Reset(event) => event.respond_to(),
            Self::Issue(event) => event.respond_to(),
            Self::Encode(event) => event.respond_to(),
        }
    }
}

impl ServiceResponder<ResetPasswordResponsePb> for EncodeAuthTokenEvent {
    fn respond_to(self) -> Result<Response<ResetPasswordResponsePb>, Status> {
        match self {
            Self::TokenExpiresCalculated(_) => {
                Err(Status::cancelled("cancelled at token expires calculated"))
            }
            Self::Success(token, granted) => (token, granted).respond_to(),
            Self::TicketNotFound => Err(Status::unauthenticated("ticket not found")),
            Self::RepositoryError(err) => err.respond_to(),
            Self::EncodeError(err) => err.respond_to(),
        }
    }
}

impl ServiceResponder<ResetPasswordResponsePb> for (AuthToken, AuthPermissionGranted) {
    fn respond_to(self) -> Result<Response<ResetPasswordResponsePb>, Status> {
        Ok(Response::new(ResetPasswordResponsePb {
            success: true,
            token: Some(self.0.into()),
            granted: Some(self.1.into()),
            ..Default::default()
        }))
    }
}

impl ServiceResponder<ResetPasswordResponsePb> for ResetPasswordEvent {
    fn respond_to(self) -> Result<Response<ResetPasswordResponsePb>, Status> {
        match self {
            Self::ResetNotified(_) => Err(Status::cancelled("cancelled at reset notified")),
            Self::Success(_) => Err(Status::cancelled("cancelled at reset password")),
            Self::Invalid(_) => Ok(invalid_reset()),
            Self::NotFound => Ok(invalid_reset()),
            Self::LoginIdNotMatched => Ok(invalid_reset()),
            Self::ResetTokenExpired => Ok(invalid_reset()),
            Self::AlreadyReset => Ok(already_reset()),
            Self::RepositoryError(err) => err.respond_to(),
            Self::PasswordHashError(err) => err.respond_to(),
            Self::DecodeError(err) => err.respond_to(),
            Self::NotifyError(err) => err.respond_to(),
        }
    }
}

fn invalid_reset() -> Response<ResetPasswordResponsePb> {
    Response::new(ResetPasswordResponsePb {
        success: false,
        err: ResetPasswordErrorKindPb::InvalidReset as i32,
        ..Default::default()
    })
}
fn already_reset() -> Response<ResetPasswordResponsePb> {
    Response::new(ResetPasswordResponsePb {
        success: false,
        err: ResetPasswordErrorKindPb::AlreadyReset as i32,
        ..Default::default()
    })
}

impl<T> ServiceResponder<T> for DecodeResetTokenError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        Err(Status::unauthenticated("failed to decode reset token"))
    }
}

impl<T> ServiceResponder<T> for NotifyResetPasswordError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        match self {
            Self::InfraError(_) => Err(Status::internal("notify reset password error")),
        }
    }
}
