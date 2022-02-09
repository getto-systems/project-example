use std::env::var;

pub struct CoreEnv {
    pub port: String,

    pub auth_service_url: String,
}

impl CoreEnv {
    pub fn new() -> Self {
        Self {
            port: load("PORT"),

            auth_service_url: load("AUTH_SERVICE_URL"),
        }
    }
}

fn load(key: &'static str) -> String {
    var(key).expect(format!("env not specified: {}", key).as_str())
}