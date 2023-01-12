use tonic::{Response, Status};

use crate::auth::user::account::modify::y_protobuf::service::{
    ModifyAuthUserAccountErrorKindPb, ModifyAuthUserAccountResponsePb,
};

use crate::common::api::response::tonic::ServiceResponder;

use super::super::action::{ModifyAuthUserAccountEvent, ModifyAuthUserAccountState};

impl ServiceResponder<ModifyAuthUserAccountResponsePb> for ModifyAuthUserAccountState {
    fn respond_to(self) -> Result<Response<ModifyAuthUserAccountResponsePb>, Status> {
        match self {
            Self::Authorize(event) => event.respond_to(),
            Self::ModifyUser(event) => event.respond_to(),
        }
    }
}

impl ServiceResponder<ModifyAuthUserAccountResponsePb> for ModifyAuthUserAccountEvent {
    fn respond_to(self) -> Result<Response<ModifyAuthUserAccountResponsePb>, Status> {
        match self {
            Self::Success => Ok(Response::new(ModifyAuthUserAccountResponsePb {
                success: true,
                ..Default::default()
            })),
            Self::NotFound => Ok(Response::new(ModifyAuthUserAccountResponsePb {
                success: false,
                err: ModifyAuthUserAccountErrorKindPb::NotFound as i32,
                ..Default::default()
            })),
            Self::Conflict => Ok(Response::new(ModifyAuthUserAccountResponsePb {
                success: false,
                err: ModifyAuthUserAccountErrorKindPb::Conflict as i32,
                ..Default::default()
            })),
            Self::Invalid(_) => Ok(Response::new(ModifyAuthUserAccountResponsePb {
                success: false,
                err: ModifyAuthUserAccountErrorKindPb::Invalid as i32,
                ..Default::default()
            })),
            Self::RepositoryError(err) => err.respond_to(),
        }
    }
}
