use tonic::{Code, Response, Status};

use crate::z_lib::response::tonic::ServiceResponder;

use crate::auth::proxy::data::AuthProxyError;

impl From<Status> for AuthProxyError {
    fn from(status: Status) -> Self {
        match status.code() {
            Code::AlreadyExists => Self::AlreadyExists(status.message().into()),
            Code::Unauthenticated => Self::Unauthenticated(status.message().into()),
            Code::PermissionDenied => Self::PermissionDenied(status.message().into()),
            Code::Cancelled => Self::Cancelled(status.message().into()),
            _ => Self::InfraError(format!("status: {}({})", status.code(), status.message())),
        }
    }
}

impl<T> ServiceResponder<T> for AuthProxyError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        match self {
            Self::AlreadyExists(message) => Err(Status::already_exists(message)),
            Self::Unauthenticated(message) => Err(Status::unauthenticated(message)),
            Self::PermissionDenied(message) => Err(Status::permission_denied(message)),
            Self::Cancelled(message) => Err(Status::cancelled(message)),
            Self::InfraError(message) => Err(Status::internal(message)),
            Self::MessageError(err) => err.respond_to(),
        }
    }
}
