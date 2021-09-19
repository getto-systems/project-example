use std::sync::Arc;

use lazy_static::lazy_static;
use tonic::{service::interceptor_fn, transport::Server};
use tower::ServiceBuilder;

use example_api::x_outside_feature::_auth::{
    env::Env,
    feature::{AppData, AppFeature},
};

lazy_static! {
    static ref ENV: Env = Env::new();
}

#[tokio::main]
async fn main() {
    let data: AppData = Arc::new(AppFeature::new(&ENV).await);

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
        .add_service(server.auth.auth_ticket.logout())
        .add_service(server.auth.auth_ticket.renew())
        .add_service(server.auth.auth_ticket.validate())
        .add_service(server.auth.password.authenticate())
        .add_service(server.auth.password.change())
        .add_service(server.auth.password.reset.request_token())
        .add_service(server.auth.password.reset.reset())
        .serve(
            format!("0.0.0.0:{}", &ENV.port)
                .parse()
                .expect("failed to parse socket addr"),
        )
        .await
        .expect("failed to start grpc server")
}

mod route {
    use example_api::auth::_auth::x_tonic::route::AuthServer;

    pub struct Server {
        pub auth: AuthServer,
    }

    impl Server {
        pub const fn new() -> Self {
            Self {
                auth: AuthServer::new(),
            }
        }
    }
}
