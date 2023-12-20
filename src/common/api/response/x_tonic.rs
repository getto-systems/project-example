use tonic::{Response, Status};

pub trait ServiceResponder<T> {
    fn respond_to(self) -> Result<Response<T>, Status>;
}

impl<T, V: ServiceResponder<T>, E: ServiceResponder<T>> ServiceResponder<T> for Result<V, E> {
    fn respond_to(self) -> Result<Response<T>, Status> {
        match self {
            Ok(value) => value.respond_to(),
            Err(err) => err.respond_to(),
        }
    }
}
