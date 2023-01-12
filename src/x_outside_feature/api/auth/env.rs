use std::env::var;

pub struct AuthEnv {
    pub log_level: String,
    pub port: String,

    pub auth_service_url: String,

    pub reset_password_url: String,
    pub cloudfront_resource: String,

    pub authenticate_private_key: String,
    pub authenticate_public_key: String,

    pub authorize_private_key: String,
    pub authorize_public_key: String,

    pub reset_token_private_key: String,
    pub reset_token_public_key: String,

    pub cloudfront_private_key: String,
    pub cloudfront_key_pair_id: String,

    pub dynamodb_auth_ticket_table: String,
    pub dynamodb_auth_user_table: String,
    pub dynamodb_auth_login_id_table: String,
    pub dynamodb_auth_reset_token_table: String,
}

impl AuthEnv {
    pub fn new() -> Self {
        Self {
            log_level: load("AUTH_LOG_LEVEL"),
            port: load("PORT"),

            auth_service_url: load("AUTH_SERVICE_URL"),

            reset_password_url: load("RESET_PASSWORD_URL"),
            cloudfront_resource: load("CLOUDFRONT_RESOURCE"),

            authenticate_private_key: load("SECRET_AUTHENTICATE_PRIVATE_KEY"),
            authenticate_public_key: load("SECRET_AUTHENTICATE_PUBLIC_KEY"),

            authorize_private_key: load("SECRET_AUTHORIZE_PRIVATE_KEY"),
            authorize_public_key: load("SECRET_AUTHORIZE_PUBLIC_KEY"),

            reset_token_private_key: load("SECRET_RESET_TOKEN_PRIVATE_KEY"),
            reset_token_public_key: load("SECRET_RESET_TOKEN_PUBLIC_KEY"),

            cloudfront_private_key: load("SECRET_CLOUDFRONT_PRIVATE_KEY"),
            cloudfront_key_pair_id: load("SECRET_CLOUDFRONT_KEY_PAIR_ID"),

            dynamodb_auth_ticket_table: load("DYNAMODB_AUTH_TICKET_TABLE"),
            dynamodb_auth_user_table: load("DYNAMODB_AUTH_USER_TABLE"),
            dynamodb_auth_login_id_table: load("DYNAMODB_AUTH_LOGIN_ID_TABLE"),
            dynamodb_auth_reset_token_table: load("DYNAMODB_AUTH_RESET_TOKEN_TABLE"),
        }
    }
}

fn load(key: &'static str) -> String {
    var(key).expect(format!("env not specified: {}", key).as_str())
}
