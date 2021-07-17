use std::io;

use lazy_static::lazy_static;
use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};

use example_api::x_outside_feature::_api::{env::Env, feature::AppFeature};

use example_api::auth::_api::x_actix_web::route::scope_auth;

lazy_static! {
    static ref ENV: Env = Env::new();
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let data = Data::new(AppFeature::new(&ENV).await);

    HttpServer::new(move || {
        let cors = Cors::default()
            .supports_credentials()
            .allowed_origin(&ENV.origin)
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE"])
            .allowed_headers(vec!["X-GETTO-EXAMPLE-NONCE"])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(data.clone())
            .service(root::index)
            .service(scope_auth())
    })
    .bind(format!("0.0.0.0:{}", &ENV.port))?
    .run()
    .await
}

mod root {
    use actix_web::{get, Responder};

    use example_api::y_environment::_api::env::VERSION;

    #[get("/")]
    async fn index() -> impl Responder {
        format!("GETTO-EXAMPLE-API: OK; version: {}", VERSION)
    }
}
