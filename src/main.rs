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
    use std::env::var;

    use actix_web::{get, Responder};
    use mysql::{params, prelude::Queryable, Error, Opts, Pool};

    #[get("/mysql")]
    async fn mysql_select() -> impl Responder {
        match select().await {
            Ok(response) => response,
            Err(err) => format!("{}", err),
        }
    }

    const USER_ID: &'static str = "admin";

    async fn select() -> Result<String, Error> {
        let pool = Pool::new(Opts::from_url(load("SECRET_MYSQL_AUTH_URL").as_str())?)?;
        let mut conn = pool.get_conn()?;

        conn.exec_drop(
            "insert into user_granted_role(user_id, role) values(:user_id, :role)",
            params! {
                "user_id" => USER_ID,
                "role" => "admin",
            },
        )?;
        conn.exec_drop(
            "insert into user_granted_role(user_id, role) values(:user_id, :role)",
            params! {
                "user_id" => USER_ID,
                "role" => "dev-docs",
            },
        )?;

        Ok(format!("success"))
    }

    fn load(key: &'static str) -> String {
        var(key).expect(format!("env not specified: {}", key).as_str())
    }
}
