use crate::avail::unexpected_error::notify::x_tonic::route::ServiceNotify;

#[derive(Default)]
pub struct UnexpectedErrorServer {
    pub notify: ServiceNotify,
}
