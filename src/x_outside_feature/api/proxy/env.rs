use std::env::var;

#[derive(Debug)]
pub struct ProxyEnv {
    pub log_level: String,
    pub port: String,

    pub domain: String,
    pub origin: String,

    pub cloudfront_key_pair_id: String,

    pub auth_service_url: String,
    pub core_service_url: String,

    pub authenticate_public_key: String,
    pub authorize_public_key: String,
}

impl ProxyEnv {
    pub fn load() -> Self {
        Self {
            log_level: load("PROXY_LOG_LEVEL"),
            port: load("PORT"),

            domain: load("DOMAIN"),
            origin: load("ORIGIN"),

            cloudfront_key_pair_id: load("SECRET_CLOUDFRONT_KEY_PAIR_ID"),

            auth_service_url: load("AUTH_SERVICE_URL"),
            core_service_url: load("CORE_SERVICE_URL"),

            authenticate_public_key: load("SECRET_AUTHENTICATE_PUBLIC_KEY"),
            authorize_public_key: load("SECRET_AUTHORIZE_PUBLIC_KEY"),
        }
    }
}

fn load(key: &'static str) -> String {
    var(key).expect(format!("env not specified: {}", key).as_str())
}
