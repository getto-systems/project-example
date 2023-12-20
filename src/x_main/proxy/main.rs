use std::{io::Result as IOResult, sync::OnceLock};

use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};

use example_api::x_outside_feature::proxy::{env::ProxyEnv, feature::ProxyAppFeature};

use example_api::{
    auth::x_actix_web::route::scope_auth, avail::x_actix_web::route::scope_avail,
    common::x_actix_web::route::scope_common,
};

static ENV: OnceLock<ProxyEnv> = OnceLock::new();

#[actix_web::main]
async fn main() -> IOResult<()> {
    ENV.set(ProxyEnv::load()).unwrap();

    let feature: Data<ProxyAppFeature> = Data::new(ProxyAppFeature::new(&ENV.get().unwrap()).await);

    HttpServer::new(move || {
        let cors = Cors::default()
            .supports_credentials()
            .allowed_origin(&ENV.get().unwrap().origin)
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE"])
            .allowed_headers(vec!["GETTO-EXAMPLE-NONCE"])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(feature.clone())
            .service(root::index)
            .service(scope_auth())
            .service(scope_avail())
            .service(scope_common())
    })
    .bind(format!("0.0.0.0:{}", &ENV.get().unwrap().port))?
    .run()
    .await
}

mod root {
    use actix_web::{get, Responder};

    use example_api::y_environment::env::VERSION;

    #[get("/")]
    async fn index() -> impl Responder {
        format!("GETTO-EXAMPLE-API: OK; version: {}", VERSION)
    }
}
