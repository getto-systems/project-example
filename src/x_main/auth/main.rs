use std::sync::Arc;

use lazy_static::lazy_static;
use tonic::{service::interceptor, transport::Server, Request};

use example_api::x_outside_feature::auth::{env::AuthEnv, feature::AuthAppFeature};

lazy_static! {
    static ref ENV: AuthEnv = AuthEnv::new();
}

#[tokio::main]
async fn main() {
    let feature: Arc<AuthAppFeature> = Arc::new(AuthAppFeature::new(&ENV).await);

    let server = route::Server::new();

    Server::builder()
        .layer(interceptor(move |mut request: Request<()>| {
            request.extensions_mut().insert(feature.clone());
            Ok(request)
        }))
        .add_service(server.auth.ticket.logout())
        .add_service(server.auth.ticket.check())
        .add_service(server.auth.ticket.validate())
        .add_service(server.auth.user.password.authenticate())
        .add_service(server.auth.user.password.change())
        .add_service(server.auth.user.password.reset.request_token())
        .add_service(server.auth.user.password.reset.reset())
        .add_service(server.auth.user.account.search())
        .serve(
            format!("0.0.0.0:{}", &ENV.port)
                .parse()
                .expect("failed to parse socket addr"),
        )
        .await
        .expect("failed to start grpc server")
}

mod route {
    use example_api::auth::x_tonic::route::AuthServer;

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