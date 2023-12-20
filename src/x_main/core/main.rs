use std::sync::{Arc, OnceLock};

use tonic::{service::interceptor, transport::Server, Request};

use example_api::x_outside_feature::core::{env::CoreEnv, feature::CoreAppFeature};

static ENV: OnceLock<CoreEnv> = OnceLock::new();

#[tokio::main]
async fn main() {
    ENV.set(CoreEnv::load()).unwrap();

    let feature: Arc<CoreAppFeature> = Arc::new(CoreAppFeature::new(&ENV.get().unwrap()));

    let server = route::Server::default();

    Server::builder()
        .layer(interceptor(move |mut request: Request<()>| {
            request.extensions_mut().insert(Arc::clone(&feature));
            Ok(request)
        }))
        .add_service(server.avail.unexpected_error.notify.server())
        .add_service(server.common.outline.load_menu_badge.server())
        .serve(
            format!("0.0.0.0:{}", &ENV.get().unwrap().port)
                .parse()
                .expect("failed to parse socket addr"),
        )
        .await
        .expect("failed to start grpc server")
}

mod route {
    use example_api::{avail::x_tonic::route::AvailServer, common::x_tonic::route::CommonServer};

    #[derive(Default)]
    pub struct Server {
        pub avail: AvailServer,
        pub common: CommonServer,
    }
}
