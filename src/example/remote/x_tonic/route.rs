use crate::example::outline::remote::x_tonic::route::OutlineServer;

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