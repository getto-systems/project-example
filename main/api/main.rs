use std::io;

use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use lazy_static::lazy_static;

use example_api::x_outside_feature::remote::api::{env::ApiEnv, feature::ApiAppFeature};

use example_api::{
    auth::remote::x_actix_web::route::scope_auth, avail::remote::x_actix_web::route::scope_avail,
    example::remote::x_actix_web::route::scope_example,
};

lazy_static! {
    static ref ENV: ApiEnv = ApiEnv::new();
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let feature: Data<ApiAppFeature> = Data::new(ApiAppFeature::new(&ENV).await);

    HttpServer::new(move || {
        let cors = Cors::default()
            .supports_credentials()
            .allowed_origin(&ENV.origin)
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE"])
            .allowed_headers(vec!["GETTO-EXAMPLE-NONCE"])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(feature.clone())
            .service(root::index)
            .service(scope_auth())
            .service(scope_avail())
            .service(scope_example())
    })
    .bind(format!("0.0.0.0:{}", &ENV.port))?
    .run()
    .await
}

mod root {
    use actix_web::{get, Responder};

    use example_api::y_environment::remote::env::VERSION;

    #[get("/")]
    async fn index() -> impl Responder {
        format!("GETTO-EXAMPLE-API: OK; version: {}", VERSION)
    }
}
