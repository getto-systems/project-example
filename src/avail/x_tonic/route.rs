use crate::avail::unexpected_error::x_tonic::route::UnexpectedErrorServer;

pub struct AvailServer {
    pub unexpected_error: UnexpectedErrorServer,
}

impl AvailServer {
    pub const fn new() -> Self {
        Self {
            unexpected_error: UnexpectedErrorServer,
        }
    }
}
