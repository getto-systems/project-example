use crate::example::outline::_example::x_tonic::route::OutlineServer;

pub struct ExampleServer {
    pub outline: OutlineServer,
}

impl ExampleServer {
    pub const fn new() -> Self {
        Self {
            outline: OutlineServer,
        }
    }
}