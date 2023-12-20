use tonic::{Code, Response, Status};

use crate::common::api::response::x_tonic::ServiceResponder;

use crate::common::proxy::data::CoreProxyCallError;

impl From<Status> for CoreProxyCallError {
    fn from(status: Status) -> Self {
        match status.code() {
            Code::PermissionDenied => Self::PermissionDenied(status.message().into()),
            _ => Self::InfraError(format!("status: {}({})", status.code(), status.message())),
        }
    }
}

impl<T> ServiceResponder<T> for CoreProxyCallError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        match self {
            Self::PermissionDenied(message) => Err(Status::permission_denied(message)),
            Self::InfraError(message) => Err(Status::internal(message)),
            Self::CheckAuthorizeTokenError(err) => err.respond_to(),
            Self::ValidateAuthorizeTokenError(err) => err.respond_to(),
            Self::ServiceConnectError(err) => err.respond_to(),
            Self::ServiceMetadataError(err) => err.respond_to(),
            Self::ServiceAuthorizeError(err) => err.respond_to(),
            Self::MessageError(err) => err.respond_to(),
        }
    }
}
