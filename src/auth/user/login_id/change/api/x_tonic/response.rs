use tonic::{Response, Status};

use crate::auth::user::login_id::change::y_protobuf::service::{
    OverrideLoginIdErrorKindPb, OverrideLoginIdResponsePb,
};
use crate::z_lib::response::tonic::ServiceResponder;

use super::super::action::{OverrideLoginIdEvent, OverrideLoginIdState};

use super::super::data::OverrideLoginIdError;

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
            Self::Failed(err) => err.respond_to(),
            Self::RepositoryError(err) => err.respond_to(),
        }
    }
}

impl ServiceResponder<OverrideLoginIdResponsePb> for OverrideLoginIdError {
    fn respond_to(self) -> Result<Response<OverrideLoginIdResponsePb>, Status> {
        match self {
            Self::InvalidLoginId(_) => Ok(Response::new(OverrideLoginIdResponsePb {
                success: false,
                err: OverrideLoginIdErrorKindPb::InvalidLoginId as i32,
            })),
            Self::UserNotFound => Ok(Response::new(OverrideLoginIdResponsePb {
                success: false,
                // ユーザーが見つからなかった場合でも invalid login id エラーを返す
                err: OverrideLoginIdErrorKindPb::InvalidLoginId as i32,
            })),
            Self::LoginIdAlreadyRegistered => Ok(Response::new(OverrideLoginIdResponsePb {
                success: false,
                err: OverrideLoginIdErrorKindPb::AlreadyRegistered as i32,
            })),
        }
    }
}
