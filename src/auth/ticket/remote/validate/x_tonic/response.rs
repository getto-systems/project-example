use tonic::{Response, Status};

use crate::auth::ticket::remote::y_protobuf::service::ValidateApiTokenResponsePb;

use crate::z_lib::remote::response::tonic::RespondTo;

use crate::auth::ticket::remote::validate::action::ValidateApiTokenState;

use crate::auth::ticket::remote::validate::method::{
    ValidateAuthNonceEvent, ValidateAuthTokenEvent,
};

impl RespondTo<ValidateApiTokenResponsePb> for ValidateApiTokenState {
    fn respond_to(self) -> Result<Response<ValidateApiTokenResponsePb>, Status> {
        match self {
            Self::Validate(event) => event.respond_to(),
            Self::Success(user) => user.respond_to(),
        }
    }
}

impl RespondTo<ValidateApiTokenResponsePb> for ValidateAuthTokenEvent {
    fn respond_to(self) -> Result<Response<ValidateApiTokenResponsePb>, Status> {
        match self {
            Self::ValidateNonce(event) => event.respond_to(),
            Self::Success(_) => Err(Status::cancelled("validate api token cancelled")),
            Self::TokenNotSent => Err(Status::unauthenticated(format!("{}", self))),
            Self::MetadataError(err) => err.respond_to(),
            Self::DecodeError(err) => err.respond_to(),
            Self::PermissionError(err) => err.respond_to(),
        }
    }
}

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
