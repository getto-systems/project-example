use jsonwebtoken::{DecodingKey, EncodingKey};
use rusoto_dynamodb::DynamoDbClient;
use rusoto_ses::SesClient;
use sqlx::MySqlPool;

use aws_cloudfront_cookie::CloudfrontKey;

use crate::auth::auth_ticket::_auth::kernel::data::{ExpansionLimitDuration, ExpireDuration};

pub struct AuthOutsideFeature {
    pub(in crate::auth) config: AuthOutsideConfig,
    pub(in crate::auth) store: AuthOutsideStore,
    pub(in crate::auth) secret: AuthOutsideSecret,
    pub(in crate::auth) email: AuthOutsideEmail,
}
pub struct AuthOutsideConfig {
    pub ticket_expires: ExpireDuration,
    pub ticket_expansion_limit: ExpansionLimitDuration,
    pub api_expires: ExpireDuration,
    pub cloudfront_expires: ExpireDuration,
    pub reset_token_expires: ExpireDuration,
}
pub struct AuthOutsideStore {
    pub dynamodb: DynamoDbClient,
    pub nonce_table_name: &'static str,
    pub mysql: MySqlPool,
}
pub struct AuthOutsideSecret {
    pub ticket: AuthOutsideJwtSecret,
    pub api: AuthOutsideJwtSecret,
    pub cloudfront: AuthOutsideCloudfrontSecret,
    pub reset_token: AuthOutsideJwtSecret,
}
pub struct AuthOutsideJwtSecret {
    pub decoding_key: DecodingKey<'static>,
    pub encoding_key: EncodingKey,
}
pub struct AuthOutsideCloudfrontSecret {
    pub key: CloudfrontKey,
    pub key_pair_id: &'static str,
    pub resource: &'static str,
}
pub struct AuthOutsideEmail {
    pub ses: SesClient,
    pub reset_password_url: &'static str,
}
