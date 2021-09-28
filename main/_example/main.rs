use std::sync::Arc;

use lazy_static::lazy_static;
use tonic::{service::interceptor_fn, transport::Server};
use tower::ServiceBuilder;

use example_api::x_outside_feature::remote::example::{
    env::ExampleEnv,
    feature::{ExampleAppData, ExampleAppFeature},
};

lazy_static! {
    static ref ENV: ExampleEnv = ExampleEnv::new();
}

#[tokio::main]
async fn main() {
    let data: ExampleAppData = Arc::new(ExampleAppFeature::new(&ENV).await);

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
        .add_service(server.avail.unexpected_error.notify())
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
    use example_api::{
        avail::remote::x_tonic::route::AvailServer,
        example::remote::x_tonic::route::ExampleServer,
    };

    pub struct Server {
        pub avail: AvailServer,
        pub example: ExampleServer,
    }

    impl Server {
        pub const fn new() -> Self {
            Self {
                avail: AvailServer::new(),
                example: ExampleServer::new(),
            }
        }
    }
}
