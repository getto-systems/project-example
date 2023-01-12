use tonic::{Response, Status};

use crate::auth::user::login_id::change::y_protobuf::service::{
    OverwriteLoginIdErrorKindPb, OverwriteLoginIdResponsePb,
};
use crate::common::api::response::tonic::ServiceResponder;

use super::super::action::{OverwriteLoginIdEvent, OverwriteLoginIdState};

impl ServiceResponder<OverwriteLoginIdResponsePb> for OverwriteLoginIdState {
    fn respond_to(self) -> Result<Response<OverwriteLoginIdResponsePb>, Status> {
        match self {
            Self::Authorize(event) => event.respond_to(),
            Self::Overwrite(event) => event.respond_to(),
        }
    }
}

impl ServiceResponder<OverwriteLoginIdResponsePb> for OverwriteLoginIdEvent {
    fn respond_to(self) -> Result<Response<OverwriteLoginIdResponsePb>, Status> {
        match self {
            Self::Success => Ok(Response::new(OverwriteLoginIdResponsePb {
                success: true,
                ..Default::default()
            })),
            Self::Invalid(_) => Ok(Response::new(OverwriteLoginIdResponsePb {
                success: false,
                err: OverwriteLoginIdErrorKindPb::Invalid as i32,
            })),
            Self::NotFound => Ok(Response::new(OverwriteLoginIdResponsePb {
                success: false,
                err: OverwriteLoginIdErrorKindPb::NotFound as i32,
            })),
            Self::AlreadyRegistered => Ok(Response::new(OverwriteLoginIdResponsePb {
                success: false,
                err: OverwriteLoginIdErrorKindPb::AlreadyRegistered as i32,
            })),
            Self::RepositoryError(err) => err.respond_to(),
        }
    }
}
