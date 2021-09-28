use tonic::{Response, Status};

use crate::z_lib::remote::response::tonic::RespondTo;

use crate::auth::ticket::remote::y_protobuf::service::RenewAuthTicketResponsePb;

use super::super::action::RenewAuthTicketState;

impl RespondTo<RenewAuthTicketResponsePb> for RenewAuthTicketState {
    fn respond_to(self) -> Result<Response<RenewAuthTicketResponsePb>, Status> {
        match self {
            Self::Validate(event) => event.respond_to(),
            Self::Encode(event) => event.respond_to(),
        }
    }
}
