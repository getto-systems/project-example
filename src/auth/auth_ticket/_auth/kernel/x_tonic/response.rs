use tonic::{Response, Status};

use crate::z_details::_common::response::tonic::RespondTo;

use super::super::data::{ValidateAuthNonceError, ValidateAuthRolesError};

impl<T> RespondTo<T> for ValidateAuthNonceError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        match self {
            Self::MetadataError(err) => err.respond_to(),
            Self::RepositoryError(err) => err.respond_to(),
            Self::Conflict => Err(Status::already_exists("nonce is already exists")),
        }
    }
}

impl<T> RespondTo<T> for ValidateAuthRolesError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        match self {
            Self::PermissionDenied(_, _) => Err(Status::permission_denied("permission denied")),
        }
    }
}
