use std::io;

use actix_cors::Cors;
use actix_web::{get, web, App, HttpServer, Responder};

use example_api::x_outside_feature::_api::init::new_app_state;

use example_api::auth::_api::x_actix_web::route::scope_auth;

#[get("/")]
async fn root() -> impl Responder {
    "GETTO-EXAMPLE-API: OK"
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let (state, setting) = new_app_state();

    let data = web::Data::new(state);
    let origin = setting.origin;

    HttpServer::new(move || {
        let cors = Cors::default()
            .supports_credentials()
            .allowed_origin(&origin)
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE"])
            .allowed_headers(vec!["X-GETTO-EXAMPLE-NONCE"])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(data.clone())
            .service(root)
            .service(scope_auth())
    })
    .bind(format!("0.0.0.0:{}", setting.port))?
    .run()
    .await
}
