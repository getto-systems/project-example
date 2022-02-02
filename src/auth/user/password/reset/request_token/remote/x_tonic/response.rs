use tonic::{Response, Status};

use crate::auth::user::password::reset::remote::y_protobuf::service::RequestResetTokenResponsePb;

use crate::z_lib::remote::response::tonic::RespondTo;

use super::super::action::{RequestResetTokenEvent, RequestResetTokenState};

use crate::auth::user::password::reset::request_token::remote::data::{
    EncodeResetTokenError, NotifyResetTokenError, RequestResetTokenError,
};

impl RespondTo<RequestResetTokenResponsePb> for RequestResetTokenState {
    fn respond_to(self) -> Result<Response<RequestResetTokenResponsePb>, Status> {
        match self {
            Self::ValidateNonce(event) => event.respond_to(),
            Self::RequestToken(event) => event.respond_to(),
        }
    }
}

impl RespondTo<RequestResetTokenResponsePb> for RequestResetTokenEvent {
    fn respond_to(self) -> Result<Response<RequestResetTokenResponsePb>, Status> {
        match self {
            Self::TokenExpiresCalculated(_) => cancelled(),
            Self::TokenNotified(_) => cancelled(),
            Self::Success => Ok(Response::new(RequestResetTokenResponsePb { success: true })),
            Self::InvalidRequest(err) => err.respond_to(),
            Self::RepositoryError(err) => err.respond_to(),
            Self::EncodeError(err) => err.respond_to(),
            Self::NotifyError(err) => err.respond_to(),
        }
    }
}

fn cancelled<T>() -> Result<Response<T>, Status> {
    Err(Status::cancelled("request reset token cancelled"))
}

impl RespondTo<RequestResetTokenResponsePb> for RequestResetTokenError {
    fn respond_to(self) -> Result<Response<RequestResetTokenResponsePb>, Status> {
        Ok(Response::new(RequestResetTokenResponsePb {
            success: false,
        }))
    }
}

impl<T> RespondTo<T> for EncodeResetTokenError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        match self {
            Self::InfraError(_) => Err(Status::internal("encode reset token error")),
        }
    }
}

impl<T> RespondTo<T> for NotifyResetTokenError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        match self {
            Self::InfraError(_) => Err(Status::internal("notify reset token error")),
        }
    }
}