use tonic::{Response, Status};

use crate::z_lib::remote::response::tonic::RespondTo;

use super::super::method::ValidateAuthNonceEvent;

impl<T> RespondTo<T> for ValidateAuthNonceEvent {
    fn respond_to(self) -> Result<Response<T>, Status> {
        match self {
            Self::NonceExpiresCalculated(_) => Err(Status::cancelled("nonce expires calculated")),
            Self::Success => Err(Status::cancelled("validate nonce succeeded")),
            Self::NonceNotSent => Err(Status::invalid_argument(format!("{}", self))),
            Self::MetadataError(err) => err.respond_to(),
            Self::RepositoryError(err) => err.respond_to(),
            Self::Conflict => Err(Status::already_exists(format!("{}", self))),
        }
    }
}
