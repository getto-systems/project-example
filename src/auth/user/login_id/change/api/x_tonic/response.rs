use tonic::{Response, Status};

use crate::auth::user::login_id::change::y_protobuf::service::{
    OverrideLoginIdErrorKindPb, OverrideLoginIdResponsePb,
};
use crate::z_lib::response::tonic::ServiceResponder;

use super::super::action::{OverrideLoginIdEvent, OverrideLoginIdState};

impl ServiceResponder<OverrideLoginIdResponsePb> for OverrideLoginIdState {
    fn respond_to(self) -> Result<Response<OverrideLoginIdResponsePb>, Status> {
        match self {
            Self::Validate(event) => event.respond_to(),
            Self::Override(event) => event.respond_to(),
        }
    }
}

impl ServiceResponder<OverrideLoginIdResponsePb> for OverrideLoginIdEvent {
    fn respond_to(self) -> Result<Response<OverrideLoginIdResponsePb>, Status> {
        match self {
            Self::Success => Ok(Response::new(OverrideLoginIdResponsePb {
                success: true,
                ..Default::default()
            })),
            Self::Invalid(_) => Ok(Response::new(OverrideLoginIdResponsePb {
                success: false,
                err: OverrideLoginIdErrorKindPb::Invalid as i32,
            })),
            Self::NotFound => Ok(Response::new(OverrideLoginIdResponsePb {
                success: false,
                err: OverrideLoginIdErrorKindPb::NotFound as i32,
            })),
            Self::AlreadyRegistered => Ok(Response::new(OverrideLoginIdResponsePb {
                success: false,
                err: OverrideLoginIdErrorKindPb::AlreadyRegistered as i32,
            })),
            Self::RepositoryError(err) => err.respond_to(),
        }
    }
}
