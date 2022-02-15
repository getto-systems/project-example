use tonic::{Response, Status};

use crate::auth::user::password::reset::request_token::y_protobuf::service::RequestResetTokenResponsePb;

use crate::z_lib::response::tonic::ServiceResponder;

use super::super::action::{RequestResetTokenEvent, RequestResetTokenState};

use crate::auth::user::password::reset::request_token::data::{
    EncodeResetTokenError, NotifyResetTokenError, RequestResetTokenError,
};

impl ServiceResponder<RequestResetTokenResponsePb> for RequestResetTokenState {
    fn respond_to(self) -> Result<Response<RequestResetTokenResponsePb>, Status> {
        match self {
            Self::ValidateNonce(event) => event.respond_to(),
            Self::RequestToken(event) => event.respond_to(),
        }
    }
}

impl ServiceResponder<RequestResetTokenResponsePb> for RequestResetTokenEvent {
    fn respond_to(self) -> Result<Response<RequestResetTokenResponsePb>, Status> {
        match self {
            Self::TokenExpiresCalculated(_) => Err(Status::cancelled(
                "cancelled at request reset token cancelled",
            )),
            Self::TokenNotified(_) => Err(Status::cancelled(
                "cancelled at request reset token notified",
            )),
            Self::Success => Ok(Response::new(RequestResetTokenResponsePb { success: true })),
            Self::InvalidRequest(err) => err.respond_to(),
            Self::RepositoryError(err) => err.respond_to(),
            Self::EncodeError(err) => err.respond_to(),
            Self::NotifyError(err) => err.respond_to(),
        }
    }
}

impl ServiceResponder<RequestResetTokenResponsePb> for RequestResetTokenError {
    fn respond_to(self) -> Result<Response<RequestResetTokenResponsePb>, Status> {
        Ok(Response::new(RequestResetTokenResponsePb {
            success: false,
        }))
    }
}

impl<T> ServiceResponder<T> for EncodeResetTokenError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        match self {
            Self::InfraError(_) => Err(Status::internal("encode reset token error")),
        }
    }
}

impl<T> ServiceResponder<T> for NotifyResetTokenError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        match self {
            Self::InfraError(_) => Err(Status::internal("notify reset token error")),
        }
    }
}
