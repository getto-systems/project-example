use tonic::{Response, Status};

use crate::auth::user::password::reset::token_destination::change::y_protobuf::service::{
    ChangeResetTokenDestinationErrorKindPb, ChangeResetTokenDestinationResponsePb,
    ResetTokenDestinationPb,
};

use crate::z_lib::response::tonic::ServiceResponder;

use super::super::action::{ChangeResetTokenDestinationEvent, ChangeResetTokenDestinationState};

use crate::auth::user::password::reset::kernel::data::ResetTokenDestinationExtract;

impl ServiceResponder<ChangeResetTokenDestinationResponsePb> for ChangeResetTokenDestinationState {
    fn respond_to(self) -> Result<Response<ChangeResetTokenDestinationResponsePb>, Status> {
        match self {
            Self::Validate(event) => event.respond_to(),
            Self::ChangeDestination(event) => event.respond_to(),
        }
    }
}

impl ServiceResponder<ChangeResetTokenDestinationResponsePb> for ChangeResetTokenDestinationEvent {
    fn respond_to(self) -> Result<Response<ChangeResetTokenDestinationResponsePb>, Status> {
        match self {
            Self::Success(destination) => {
                Ok(Response::new(ChangeResetTokenDestinationResponsePb {
                    success: true,
                    data: Some(match destination.extract() {
                        ResetTokenDestinationExtract::None => ResetTokenDestinationPb {
                            r#type: "none".into(),
                            ..Default::default()
                        },
                        ResetTokenDestinationExtract::Email(email) => ResetTokenDestinationPb {
                            r#type: "email".into(),
                            email,
                        },
                    }),
                    ..Default::default()
                }))
            }
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
