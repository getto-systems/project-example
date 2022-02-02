use tonic::{Response, Status};

use crate::z_lib::remote::response::tonic::RespondTo;

use crate::auth::user::password::kernel::data::PasswordHashError;

impl<T> RespondTo<T> for PasswordHashError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        Err(Status::internal(format!("password hash error; {}", self)))
    }
}
