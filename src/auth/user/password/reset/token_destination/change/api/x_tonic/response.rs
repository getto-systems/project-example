use tonic::{Response, Status};

use crate::auth::user::password::reset::token_destination::change::y_protobuf::service::{
    ChangeResetTokenDestinationErrorKindPb, ChangeResetTokenDestinationResponsePb,
};

use crate::common::api::response::tonic::ServiceResponder;

use super::super::action::{ChangeResetTokenDestinationEvent, ChangeResetTokenDestinationState};

impl ServiceResponder<ChangeResetTokenDestinationResponsePb> for ChangeResetTokenDestinationState {
    fn respond_to(self) -> Result<Response<ChangeResetTokenDestinationResponsePb>, Status> {
        match self {
            Self::Authorize(event) => event.respond_to(),
            Self::ChangeDestination(event) => event.respond_to(),
        }
    }
}

impl ServiceResponder<ChangeResetTokenDestinationResponsePb> for ChangeResetTokenDestinationEvent {
    fn respond_to(self) -> Result<Response<ChangeResetTokenDestinationResponsePb>, Status> {
        match self {
            Self::Success => Ok(Response::new(ChangeResetTokenDestinationResponsePb {
                success: true,
                ..Default::default()
            })),
            Self::NotFound => Ok(Response::new(ChangeResetTokenDestinationResponsePb {
                success: false,
                err: ChangeResetTokenDestinationErrorKindPb::NotFound as i32,
                ..Default::default()
            })),
            Self::Conflict => Ok(Response::new(ChangeResetTokenDestinationResponsePb {
                success: false,
                err: ChangeResetTokenDestinationErrorKindPb::Conflict as i32,
                ..Default::default()
            })),
            Self::Invalid(_) => Ok(Response::new(ChangeResetTokenDestinationResponsePb {
                success: false,
                err: ChangeResetTokenDestinationErrorKindPb::Invalid as i32,
                ..Default::default()
            })),
            Self::RepositoryError(err) => err.respond_to(),
        }
    }
}
