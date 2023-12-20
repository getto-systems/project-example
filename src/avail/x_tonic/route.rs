use crate::avail::unexpected_error::x_tonic::route::UnexpectedErrorServer;

#[derive(Default)]
pub struct AvailServer {
    pub unexpected_error: UnexpectedErrorServer,
}
