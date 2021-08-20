use tonic::{Code, Status};

use crate::example::_api::service::data::ExampleServiceError;

impl From<Status> for ExampleServiceError {
    fn from(status: Status) -> Self {
        match status.code() {
            Code::Internal => Self::Internal(status.message().into()),
            Code::Cancelled => Self::Cancelled(status.message().into()),
            _ => Self::InfraError(status.message().into()),
        }
    }
}
