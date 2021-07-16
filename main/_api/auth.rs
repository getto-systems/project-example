use std::env::var;

use tonic::transport::Server;

#[tokio::main]
async fn main() {
    let port = var("PORT").expect("PORT is not specified");

    Server::builder()
        .add_service(demo::new_greeter_server())
        .serve(
            format!("0.0.0.0:{}", port)
                .parse()
                .expect("failed to parse socket addr"),
        )
        .await
        .expect("failed to start grpc server")
}

mod demo {
    use tonic::Response;

    use example_api::auth::_api::y_grpc::service::{
        greeter_server::{Greeter, GreeterServer},
        HelloResponse,
    };

    pub fn new_greeter_server() -> GreeterServer<DemoGreeter> {
        GreeterServer::new(DemoGreeter)
    }

    pub struct DemoGreeter;

    #[async_trait::async_trait]
    impl Greeter for DemoGreeter {
        async fn hello(
            &self,
            request: tonic::Request<example_api::auth::_api::y_grpc::service::HelloRequest>,
        ) -> Result<
            tonic::Response<example_api::auth::_api::y_grpc::service::HelloResponse>,
            tonic::Status,
        > {
            println!("hello request received; {:?}", request);

            Ok(Response::new(HelloResponse {
                message: format!("Hello, {}!", request.into_inner().name).into(),
            }))
        }
    }
}
