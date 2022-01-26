use tonic::{Response, Status};

use crate::auth::ticket::remote::y_protobuf::service::ValidateApiTokenResponsePb;

use crate::z_lib::remote::response::tonic::RespondTo;

use crate::auth::ticket::remote::validate::method::ValidateAuthTokenEvent;

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
