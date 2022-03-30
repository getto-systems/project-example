use tonic::{Response, Status};

use crate::auth::user::account::modify::y_protobuf::service::{
    ModifyAuthUserAccountDataPb, ModifyAuthUserAccountErrorKindPb, ModifyAuthUserAccountResponsePb,
};

use crate::z_lib::response::tonic::ServiceResponder;

use super::super::action::{ModifyAuthUserAccountEvent, ModifyAuthUserAccountState};

impl ServiceResponder<ModifyAuthUserAccountResponsePb> for ModifyAuthUserAccountState {
    fn respond_to(self) -> Result<Response<ModifyAuthUserAccountResponsePb>, Status> {
        match self {
            Self::Validate(event) => event.respond_to(),
            Self::ModifyUser(event) => event.respond_to(),
        }
    }
}

impl ServiceResponder<ModifyAuthUserAccountResponsePb> for ModifyAuthUserAccountEvent {
    fn respond_to(self) -> Result<Response<ModifyAuthUserAccountResponsePb>, Status> {
        match self {
            Self::Success(user) => Ok(Response::new(ModifyAuthUserAccountResponsePb {
                success: true,
                data: Some(ModifyAuthUserAccountDataPb {
                    granted_roles: user.granted_roles.extract().into_iter().collect(),
                }),
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
            Self::InvalidUser(_) => Ok(Response::new(ModifyAuthUserAccountResponsePb {
                success: false,
                err: ModifyAuthUserAccountErrorKindPb::Invalid as i32,
                ..Default::default()
            })),
            Self::RepositoryError(err) => err.respond_to(),
        }
    }
}
