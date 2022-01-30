use tonic::{Code, Status};

use crate::auth::remote::proxy::data::AuthProxyError;

impl From<Status> for AuthProxyError {
    fn from(status: Status) -> Self {
        match status.code() {
            Code::InvalidArgument => Self::InvalidArgument(status.message().into()),
            Code::AlreadyExists => Self::AlreadyExists(status.message().into()),
            Code::Unauthenticated => Self::Unauthenticated(status.message().into()),
            Code::PermissionDenied => Self::PermissionDenied(status.message().into()),
            Code::Internal => Self::Internal(status.message().into()),
            Code::Cancelled => Self::Cancelled(status.message().into()),
            _ => Self::InfraError(status.message().into()),
        }
    }
}
