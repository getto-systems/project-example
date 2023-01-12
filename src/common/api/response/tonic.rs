use tonic::{Response, Status};

pub trait ServiceResponder<T> {
    fn respond_to(self) -> Result<Response<T>, Status>;
}
