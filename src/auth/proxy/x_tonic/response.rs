use tonic::{Code, Response, Status};

use crate::common::api::response::tonic::ServiceResponder;

use crate::auth::proxy::data::AuthProxyError;

impl From<Status> for AuthProxyError {
    fn from(status: Status) -> Self {
        match status.code() {
            Code::Unauthenticated => Self::Unauthenticated(status.message().into()),
            _ => Self::InfraError(format!("status: {}({})", status.code(), status.message())),
        }
    }
}

impl<T> ServiceResponder<T> for AuthProxyError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        match self {
            Self::Unauthenticated(message) => Err(Status::unauthenticated(message)),
            Self::InfraError(message) => Err(Status::internal(message)),
            Self::ServiceConnectError(err) => err.respond_to(),
            Self::ServiceMetadataError(err) => err.respond_to(),
            Self::MessageError(err) => err.respond_to(),
        }
    }
}
