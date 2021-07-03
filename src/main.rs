use std::io;

use lazy_static::lazy_static;

use actix_cors::Cors;
use actix_web::{App, HttpServer};

use example_api::x_outside_feature::_api::state::AppData;
use example_api::x_outside_feature::_api::{env::Env, state::AppState};

use example_api::auth::_api::x_actix_web::route::scope_auth;

lazy_static! {
    static ref ENV: Env = Env::new();
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let state = AppState::new(&ENV);

    let data: AppData = state.into();

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
            .service(demo::mysql_select)
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

mod demo {
    use std::{env::var, path::Path};

    use actix_web::{get, Responder};
    use mysql::{prelude::Queryable, OptsBuilder, Pool, SslOpts};

    #[get("/mysql")]
    async fn mysql_select() -> impl Responder {
        match select().await {
            Ok(response) => response,
            Err(err) => err,
        }
    }

    async fn select() -> Result<String, String> {
        let opts = OptsBuilder::new()
            .ip_or_hostname(Some(load("MYSQL_SERVER")))
            .tcp_port(load_u16("MYSQL_PORT"))
            .db_name(Some(load("MYSQL_AUTH_DATABASE")))
            .user(Some(load("MYSQL_AUTH_USER")))
            .pass(Some(load("MYSQL_AUTH_PASSWORD")));

        let pool = Pool::new(opts).map_err(|err| format!("{}", err))?;
        let mut conn = pool.get_conn().expect("failed to get connection!");
        let result: u8 = conn
            .query_first("select 1")
            .expect("failed to query!")
            .expect("unexpected result!");

        Ok(format!("success; value: {}", result))
    }

    fn load(key: &'static str) -> String {
        var(key).expect(format!("env not specified: {}", key).as_str())
    }
    fn load_u16(key: &'static str) -> u16 {
        load(key).parse::<u16>().expect("u16 parse error")
    }
}
