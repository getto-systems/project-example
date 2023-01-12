use tonic::{Code, Response, Status};

use crate::common::api::response::tonic::ServiceResponder;

use crate::common::proxy::event::ProxyCallEvent;

use crate::common::proxy::data::CoreProxyError;

impl From<Status> for CoreProxyError {
    fn from(status: Status) -> Self {
        match status.code() {
            Code::PermissionDenied => Self::PermissionDenied(status.message().into()),
            _ => Self::InfraError(format!("status: {}({})", status.code(), status.message())),
        }
    }
}

impl<T, R, E: ServiceResponder<T>> ServiceResponder<T> for ProxyCallEvent<R, E> {
    fn respond_to(self) -> Result<Response<T>, Status> {
        match self {
            Self::TryToCall(message) => Err(Status::cancelled(format!(
                "cancelled at proxy call: {}",
                message
            ))),
            Self::Response(_) => Err(Status::cancelled("cancelled at proxy call succeeded")),
            Self::ServiceError(err) => err.respond_to(),
        }
    }
}

impl<T> ServiceResponder<T> for CoreProxyError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        match self {
            Self::PermissionDenied(message) => Err(Status::permission_denied(message)),
            Self::InfraError(message) => Err(Status::internal(message)),
            Self::ServiceConnectError(err) => err.respond_to(),
            Self::ServiceMetadataError(err) => err.respond_to(),
            Self::MessageError(err) => err.respond_to(),
        }
    }
}
