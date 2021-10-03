use tonic::{Response, Status};

use crate::auth::user::password::reset::remote::y_protobuf::service::{
    ResetPasswordErrorKindPb, ResetPasswordResponsePb,
};

use crate::z_lib::remote::response::tonic::RespondTo;

use super::super::event::ResetPasswordEvent;

use crate::auth::user::password::{
    remote::kernel::data::VerifyResetTokenEntryError,
    reset::remote::reset::data::{
        DecodeResetTokenError, NotifyResetPasswordError, ResetPasswordError,
    },
};

impl RespondTo<ResetPasswordResponsePb> for ResetPasswordEvent {
    fn respond_to(self) -> Result<Response<ResetPasswordResponsePb>, Status> {
        match self {
            Self::ResetNotified(_) => Err(Status::cancelled("reset password cancelled")),
            Self::Success(_) => Err(Status::cancelled("reset password cancelled")),
            Self::InvalidReset(err) => err.respond_to(),
            Self::UserNotFound => Err(Status::internal("user not found")),
            Self::NonceError(err) => err.respond_to(),
            Self::RepositoryError(err) => err.respond_to(),
            Self::PasswordHashError(err) => err.respond_to(),
            Self::DecodeError(err) => err.respond_to(),
            Self::NotifyError(err) => err.respond_to(),
        }
    }
}

impl RespondTo<ResetPasswordResponsePb> for ResetPasswordError {
    fn respond_to(self) -> Result<Response<ResetPasswordResponsePb>, Status> {
        let error: ResetPasswordErrorKindPb = self.into();
        Ok(Response::new(ResetPasswordResponsePb {
            success: false,
            error: error as i32,
            ..Default::default()
        }))
    }
}

impl Into<ResetPasswordErrorKindPb> for ResetPasswordError {
    fn into(self) -> ResetPasswordErrorKindPb {
        match self {
            Self::InvalidLoginId(_) => ResetPasswordErrorKindPb::InvalidReset,
            Self::InvalidPassword(_) => ResetPasswordErrorKindPb::InvalidReset,
            Self::InvalidResetToken(_) => ResetPasswordErrorKindPb::InvalidReset,
            Self::InvalidResetTokenEntry(err) => err.into(),
        }
    }
}

impl Into<ResetPasswordErrorKindPb> for VerifyResetTokenEntryError {
    fn into(self) -> ResetPasswordErrorKindPb {
        match self {
            Self::ResetTokenEntryNotFound => ResetPasswordErrorKindPb::InvalidReset,
            Self::LoginIdNotMatched => ResetPasswordErrorKindPb::InvalidReset,
            Self::Expired => ResetPasswordErrorKindPb::InvalidReset,
            Self::AlreadyReset => ResetPasswordErrorKindPb::AlreadyReset,
        }
    }
}

impl<T> RespondTo<T> for DecodeResetTokenError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        Err(Status::unauthenticated("failed to decode reset token"))
    }
}

impl<T> RespondTo<T> for NotifyResetPasswordError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        match self {
            Self::InfraError(_) => Err(Status::internal("notify reset password error")),
        }
    }
}
