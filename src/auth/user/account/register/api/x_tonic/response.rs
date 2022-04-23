use tonic::{Response, Status};

use crate::auth::user::account::register::y_protobuf::service::{
    RegisterAuthUserAccountErrorKindPb, RegisterAuthUserAccountResponsePb,
};

use crate::z_lib::response::tonic::ServiceResponder;

use super::super::action::{RegisterAuthUserAccountEvent, RegisterAuthUserAccountState};

impl ServiceResponder<RegisterAuthUserAccountResponsePb> for RegisterAuthUserAccountState {
    fn respond_to(self) -> Result<Response<RegisterAuthUserAccountResponsePb>, Status> {
        match self {
            Self::Validate(event) => event.respond_to(),
            Self::PermissionError(err) => err.respond_to(),
            Self::RegisterUser(event) => event.respond_to(),
        }
    }
}

impl ServiceResponder<RegisterAuthUserAccountResponsePb> for RegisterAuthUserAccountEvent {
    fn respond_to(self) -> Result<Response<RegisterAuthUserAccountResponsePb>, Status> {
        match self {
            Self::Success => Ok(Response::new(RegisterAuthUserAccountResponsePb {
                success: true,
                ..Default::default()
            })),
            Self::LoginIdAlreadyRegistered => {
                Ok(Response::new(RegisterAuthUserAccountResponsePb {
                    success: false,
                    err: RegisterAuthUserAccountErrorKindPb::LoginIdAlreadyRegistered as i32,
                    ..Default::default()
                }))
            }
            Self::Invalid(_) => Ok(Response::new(RegisterAuthUserAccountResponsePb {
                success: false,
                err: RegisterAuthUserAccountErrorKindPb::Invalid as i32,
                ..Default::default()
            })),
            Self::RepositoryError(err) => err.respond_to(),
        }
    }
}
