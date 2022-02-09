use tonic::{Response, Status};

use crate::z_lib::response::tonic::ServiceResponder;

use crate::auth::ticket::encode::data::EncodeAuthTokenError;

impl<T> ServiceResponder<T> for EncodeAuthTokenError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        Err(Status::internal(format!("{}", self)))
    }
}
