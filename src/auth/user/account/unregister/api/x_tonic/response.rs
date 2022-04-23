use tonic::{Response, Status};

use crate::auth::user::account::unregister::y_protobuf::service::UnregisterAuthUserAccountResponsePb;

use crate::z_lib::response::tonic::ServiceResponder;

use super::super::action::{UnregisterAuthUserAccountEvent, UnregisterAuthUserAccountState};

impl ServiceResponder<UnregisterAuthUserAccountResponsePb> for UnregisterAuthUserAccountState {
    fn respond_to(self) -> Result<Response<UnregisterAuthUserAccountResponsePb>, Status> {
        match self {
            Self::Validate(event) => event.respond_to(),
            Self::PermissionError(err) => err.respond_to(),
            Self::UnregisterUser(event) => event.respond_to(),
        }
    }
}

impl ServiceResponder<UnregisterAuthUserAccountResponsePb> for UnregisterAuthUserAccountEvent {
    fn respond_to(self) -> Result<Response<UnregisterAuthUserAccountResponsePb>, Status> {
        match self {
            Self::Success => Ok(Response::new(UnregisterAuthUserAccountResponsePb {
                success: true,
                ..Default::default()
            })),
            Self::Invalid(_) => Ok(Response::new(UnregisterAuthUserAccountResponsePb {
                success: false,
                ..Default::default()
            })),
            Self::RepositoryError(err) => err.respond_to(),
        }
    }
}
