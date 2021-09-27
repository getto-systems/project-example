use tonic::{Response, Status};

use crate::z_details::_common::response::tonic::RespondTo;

use super::super::data::ValidateAuthRolesError;

impl<T> RespondTo<T> for ValidateAuthRolesError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        match self {
            Self::PermissionDenied(_, _) => Err(Status::permission_denied(format!("{}", self))),
        }
    }
}
