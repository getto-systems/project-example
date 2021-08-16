use tonic::{Response, Status};

use crate::z_details::_common::response::tonic::RespondTo;

use super::super::data::{ValidateAuthNonceError, ValidateAuthRolesError};

impl<T> RespondTo<T> for ValidateAuthNonceError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        match self {
            Self::NonceNotSent => Err(Status::invalid_argument(format!("{}", self))),
            Self::MetadataError(err) => err.respond_to(),
            Self::RepositoryError(err) => err.respond_to(),
            Self::Conflict => Err(Status::already_exists(format!("{}", self))),
        }
    }
}

impl<T> RespondTo<T> for ValidateAuthRolesError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        match self {
            Self::PermissionDenied(_, _) => Err(Status::permission_denied(format!("{}", self))),
        }
    }
}
