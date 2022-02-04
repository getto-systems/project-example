use crate::core::outline::x_tonic::route::OutlineServer;

pub struct CoreServer {
    pub outline: OutlineServer,
}

impl CoreServer {
    pub const fn new() -> Self {
        Self {
            outline: OutlineServer,
        }
    }
}
