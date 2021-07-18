use tonic::{Response, Status};

pub trait RespondTo<T> {
    fn respond_to(self) -> Result<Response<T>, Status>;
}
