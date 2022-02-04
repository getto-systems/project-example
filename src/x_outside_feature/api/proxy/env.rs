use std::env::var;

pub struct ProxyEnv {
    pub port: String,

    pub domain: String,
    pub origin: String,

    pub cloudfront_key_pair_id: String,

    pub auth_service_url: String,
    pub core_service_url: String,

    pub ticket_public_key: String,
    pub api_public_key: String,
}

impl ProxyEnv {
    pub fn new() -> Self {
        Self {
            port: load("PORT"),

            domain: load("DOMAIN"),
            origin: load("ORIGIN"),

            cloudfront_key_pair_id: load("SECRET_CLOUDFRONT_KEY_PAIR_ID"),

            auth_service_url: load("AUTH_SERVICE_URL"),
            core_service_url: load("CORE_SERVICE_URL"),

            ticket_public_key: load("SECRET_TICKET_PUBLIC_KEY"),
            api_public_key: load("SECRET_API_PUBLIC_KEY"),
        }
    }
}

fn load(key: &'static str) -> String {
    var(key).expect(format!("env not specified: {}", key).as_str())
}
