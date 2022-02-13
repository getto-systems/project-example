use std::sync::Arc;

use lazy_static::lazy_static;
use tonic::{service::interceptor, transport::Server, Request};

use example_api::x_outside_feature::core::{
    env::CoreEnv, feature::CoreAppFeature,
};

lazy_static! {
    static ref ENV: CoreEnv = CoreEnv::new();
}

#[tokio::main]
async fn main() {
    let feature: Arc<CoreAppFeature> = Arc::new(CoreAppFeature::new(&ENV));

    let server = route::Server::new();

    Server::builder()
        .layer(interceptor(move |mut request: Request<()>| {
            request.extensions_mut().insert(feature.clone());
            Ok(request)
        }))
        .add_service(server.avail.unexpected_error.notify())
        .add_service(server.core.outline.load_menu_badge())
        .serve(
            format!("0.0.0.0:{}", &ENV.port)
                .parse()
                .expect("failed to parse socket addr"),
        )
        .await
        .expect("failed to start grpc server")
}

mod route {
    use example_api::{
        avail::x_tonic::route::AvailServer, core::x_tonic::route::CoreServer,
    };

    pub struct Server {
        pub avail: AvailServer,
        pub core: CoreServer,
    }

    impl Server {
        pub const fn new() -> Self {
            Self {
                avail: AvailServer::new(),
                core: CoreServer::new(),
            }
        }
    }
}
