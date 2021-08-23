use std::env::var;

pub struct Env {
    pub port: String,

    pub reset_password_url: String,
    pub cloudfront_resource: String,

    pub ticket_private_key: String,
    pub ticket_public_key: String,

    pub api_private_key: String,
    pub api_public_key: String,

    pub reset_token_private_key: String,
    pub reset_token_public_key: String,

    pub cloudfront_private_key: String,
    pub cloudfront_key_pair_id: String,

    pub dynamodb_auth_nonce_table: String,
    pub mysql_auth_url: String,
}

impl Env {
    pub fn new() -> Self {
        Self {
            port: load("PORT"),

            reset_password_url: load("RESET_PASSWORD_URL"),
            cloudfront_resource: load("CLOUDFRONT_RESOURCE"),

            ticket_private_key: load("SECRET_TICKET_PRIVATE_KEY"),
            ticket_public_key: load("SECRET_TICKET_PUBLIC_KEY"),

            api_private_key: load("SECRET_API_PRIVATE_KEY"),
            api_public_key: load("SECRET_API_PUBLIC_KEY"),

            reset_token_private_key: load("SECRET_RESET_TOKEN_PRIVATE_KEY"),
            reset_token_public_key: load("SECRET_RESET_TOKEN_PUBLIC_KEY"),

            cloudfront_private_key: load("SECRET_CLOUDFRONT_PRIVATE_KEY"),
            cloudfront_key_pair_id: load("SECRET_CLOUDFRONT_KEY_PAIR_ID"),

            dynamodb_auth_nonce_table: load("SECRET_DYNAMODB_AUTH_NONCE_TABLE"),
            mysql_auth_url: load("SECRET_MYSQL_AUTH_URL"),
        }
    }
}

fn load(key: &'static str) -> String {
    var(key).expect(format!("env not specified: {}", key).as_str())
}