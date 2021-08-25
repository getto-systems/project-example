use std::env::var;

pub struct Env {
    pub port: String,

    pub domain: String,
    pub origin: String,

    pub cloudfront_resource: String,
    pub cloudfront_key_pair_id: String,

    pub auth_service_url: String,
    pub domain_service_url: String,
}

impl Env {
    pub fn new() -> Self {
        Self {
            port: load("PORT"),

            domain: load("DOMAIN"),
            origin: load("ORIGIN"),

            cloudfront_resource: load("CLOUDFRONT_RESOURCE"),
            cloudfront_key_pair_id: load("SECRET_CLOUDFRONT_KEY_PAIR_ID"),

            auth_service_url: load("AUTH_SERVICE_URL"),
            domain_service_url: load("DOMAIN_SERVICE_URL"),
        }
    }
}

fn load(key: &'static str) -> String {
    var(key).expect(format!("env not specified: {}", key).as_str())
}
