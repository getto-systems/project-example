use std::env::var;

pub struct CoreEnv {
    pub log_level: String,
    pub port: String,

    pub auth_service_url: String,

    pub authorize_public_key: String,
}

impl CoreEnv {
    pub fn new() -> Self {
        Self {
            log_level: load("CORE_LOG_LEVEL"),
            port: load("PORT"),

            auth_service_url: load("AUTH_SERVICE_URL"),

            authorize_public_key: load("SECRET_AUTHORIZE_PUBLIC_KEY"),
        }
    }
}

fn load(key: &'static str) -> String {
    var(key).expect(format!("env not specified: {}", key).as_str())
}
