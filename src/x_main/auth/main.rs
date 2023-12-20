use std::sync::{Arc, OnceLock};

use tonic::{service::interceptor, transport::Server, Request};

use example_api::{
    auth::x_tonic::route::AuthServer,
    x_outside_feature::auth::{env::AuthEnv, feature::AuthAppFeature},
};

static ENV: OnceLock<AuthEnv> = OnceLock::new();

#[tokio::main]
async fn main() {
    ENV.set(AuthEnv::load()).unwrap();

    let feature: Arc<AuthAppFeature> = Arc::new(AuthAppFeature::new(&ENV.get().unwrap()).await);

    let server = AuthServer::default();

    Server::builder()
        .layer(interceptor(move |mut request: Request<()>| {
            request.extensions_mut().insert(Arc::clone(&feature));
            Ok(request)
        }))
        .add_service(server.ticket.logout.server())
        .add_service(server.ticket.authenticate_with_token.server())
        .add_service(server.ticket.authorize.server())
        .add_service(server.user.login_id.overwrite.server())
        .add_service(server.user.password.authenticate.server())
        .add_service(server.user.password.change.server())
        .add_service(server.user.password.overwrite.server())
        .add_service(server.user.password.reset.request_token.server())
        .add_service(server.user.password.reset.reset.server())
        .add_service(server.user.password.reset.token_destination.change.server())
        .add_service(server.user.account.search())
        .add_service(server.user.account.modify.server())
        .add_service(server.user.account.register.server())
        .add_service(server.user.account.unregister.server())
        .serve(
            format!("0.0.0.0:{}", &ENV.get().unwrap().port)
                .parse()
                .expect("failed to parse socket addr"),
        )
        .await
        .expect("failed to start grpc server")
}
