use tonic::{Code, Status};

use crate::outline::_api::service::data::OutlineServiceError;

impl From<Status> for OutlineServiceError {
    fn from(status: Status) -> Self {
        match status.code() {
            Code::Internal => Self::Internal(status.message().into()),
            Code::Cancelled => Self::Cancelled(status.message().into()),
            _ => Self::InfraError(status.message().into()),
        }
    }
}
