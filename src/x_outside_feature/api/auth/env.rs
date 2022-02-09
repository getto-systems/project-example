use std::env::var;

pub struct AuthEnv {
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
    pub dynamodb_auth_ticket_table: String,
    pub dynamodb_auth_user_table: String,
    pub dynamodb_auth_login_id_table: String,
    pub dynamodb_auth_reset_token_destination_table: String,
    pub dynamodb_auth_reset_token_table: String,
}

impl AuthEnv {
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

            dynamodb_auth_nonce_table: load("DYNAMODB_AUTH_NONCE_TABLE"),
            dynamodb_auth_ticket_table: load("DYNAMODB_AUTH_TICKET_TABLE"),
            dynamodb_auth_user_table: load("DYNAMODB_AUTH_USER_TABLE"),
            dynamodb_auth_login_id_table: load("DYNAMODB_AUTH_LOGIN_ID_TABLE"),
            dynamodb_auth_reset_token_destination_table: load(
                "DYNAMODB_AUTH_RESET_TOKEN_DESTINATION_TABLE",
            ),
            dynamodb_auth_reset_token_table: load("DYNAMODB_AUTH_RESET_TOKEN_TABLE"),
        }
    }
}

fn load(key: &'static str) -> String {
    var(key).expect(format!("env not specified: {}", key).as_str())
}
