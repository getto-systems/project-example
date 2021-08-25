use std::sync::Arc;

use lazy_static::lazy_static;
use tonic::{service::interceptor_fn, transport::Server};
use tower::ServiceBuilder;

use example_api::x_outside_feature::_example::{
    env::Env,
    feature::{AppData, AppFeature},
};

lazy_static! {
    static ref ENV: Env = Env::new();
}

#[tokio::main]
async fn main() {
    let data: AppData = Arc::new(AppFeature::new().await);

    let server = route::Server::new();

    Server::builder()
        .layer(
            ServiceBuilder::new()
                .layer(interceptor_fn(move |mut request| {
                    request.extensions_mut().insert(data.clone());
                    Ok(request)
                }))
                .into_inner(),
        )
        .add_service(server.example.outline.get_menu_badge())
        .serve(
            format!("0.0.0.0:{}", &ENV.port)
                .parse()
                .expect("failed to parse socket addr"),
        )
        .await
        .expect("failed to start grpc server")
}

mod route {
    use example_api::example::_example::x_tonic::route::ExampleServer;

    pub struct Server {
        pub example: ExampleServer,
    }

    impl Server {
        pub const fn new() -> Self {
            Self {
                example: ExampleServer::new(),
            }
        }
    }
}
