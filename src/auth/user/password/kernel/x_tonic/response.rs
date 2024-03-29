use tonic::{Response, Status};

use crate::common::api::response::x_tonic::ServiceResponder;

use crate::auth::user::password::kernel::data::PasswordHashError;

impl<T> ServiceResponder<T> for PasswordHashError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        Err(Status::internal(format!("password hash error; {}", self)))
    }
}
