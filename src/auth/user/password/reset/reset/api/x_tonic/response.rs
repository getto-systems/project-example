use tonic::{Response, Status};

use crate::auth::ticket::y_protobuf::service::EncodedAuthTokensPb;

use crate::z_lib::response::tonic::ServiceResponder;

use crate::auth::user::password::reset::reset::y_protobuf::service::{
    ResetPasswordErrorKindPb, ResetPasswordMaskedResponsePb, ResetPasswordResponsePb,
};

use super::super::action::{ResetPasswordEvent, ResetPasswordState};

use crate::auth::ticket::encode::method::EncodeAuthTicketEvent;

use crate::auth::{
    ticket::encode::data::AuthTicketEncoded,
    user::password::reset::reset::data::{DecodeResetTokenError, NotifyResetPasswordError},
};

impl ServiceResponder<ResetPasswordResponsePb> for ResetPasswordState {
    fn respond_to(self) -> Result<Response<ResetPasswordResponsePb>, Status> {
        match self {
            Self::ValidateNonce(event) => event.respond_to(),
            Self::Reset(event) => event.respond_to(),
            Self::Issue(event) => event.respond_to(),
            Self::Encode(event) => event.respond_to(),
        }
    }
}

impl ServiceResponder<ResetPasswordResponsePb> for EncodeAuthTicketEvent {
    fn respond_to(self) -> Result<Response<ResetPasswordResponsePb>, Status> {
        match self {
            Self::TokenExpiresCalculated(_) => {
                Err(Status::cancelled("cancelled at token expires calculated"))
            }
            Self::Success(response) => response.respond_to(),
            Self::TicketNotFound => Err(Status::unauthenticated("ticket not found")),
            Self::RepositoryError(err) => err.respond_to(),
            Self::EncodeError(err) => err.respond_to(),
        }
    }
}

impl ServiceResponder<ResetPasswordResponsePb> for AuthTicketEncoded {
    fn respond_to(self) -> Result<Response<ResetPasswordResponsePb>, Status> {
        Ok(Response::new(ResetPasswordResponsePb {
            success: true,
            roles: Some(self.roles.into()),
            token: Some(self.token.into()),
            ..Default::default()
        }))
    }
}

impl ServiceResponder<ResetPasswordResponsePb> for ResetPasswordEvent {
    fn respond_to(self) -> Result<Response<ResetPasswordResponsePb>, Status> {
        match self {
            Self::ResetNotified(_) => Err(Status::cancelled("cancelled at reset notified")),
            Self::Success(_) => Err(Status::cancelled("cancelled at reset password")),
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
            Self::LoginIdNotMatched => Ok(Response::new(ResetPasswordResponsePb {
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

impl ResetPasswordResponsePb {
    pub fn extract(self) -> (Option<EncodedAuthTokensPb>, ResetPasswordMaskedResponsePb) {
        (
            self.token,
            ResetPasswordMaskedResponsePb {
                success: self.success,
                roles: self.roles,
                err: self.err,
            },
        )
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
            Self::InfraError(_) => Err(Status::internal("notify reset password error")),
        }
    }
}
