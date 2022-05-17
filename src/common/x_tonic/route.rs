use crate::common::outline::x_tonic::route::OutlineServer;

pub struct CommonServer {
    pub outline: OutlineServer,
}

impl CommonServer {
    pub const fn new() -> Self {
        Self {
            outline: OutlineServer,
        }
    }
}
