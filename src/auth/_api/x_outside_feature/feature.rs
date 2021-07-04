use jsonwebtoken::{DecodingKey, EncodingKey};
use mysql::Pool;
use rusoto_dynamodb::DynamoDbClient;
use rusoto_ses::SesClient;

use aws_cloudfront_cookie::CloudfrontKey;

use crate::auth::auth_ticket::_api::kernel::data::{ExpansionLimitDuration, ExpireDuration};

pub struct AuthOutsideFeature {
    pub(in crate::auth) config: AuthOutsideConfig,
    pub(in crate::auth) store: AuthOutsideStore,
    pub(in crate::auth) cookie: AuthOutsideCookie,
    pub(in crate::auth) secret: AuthOutsideSecret,
    pub(in crate::auth) email: AuthOutsideEmail,
}
pub struct AuthOutsideConfig {
    pub ticket_expires: ExpireDuration,
    pub ticket_expansion_limit: ExpansionLimitDuration,
    pub api_expires: ExpireDuration,
    pub cdn_expires: ExpireDuration,
    pub reset_token_expires: ExpireDuration,
}
pub struct AuthOutsideStore {
    pub dynamodb: DynamoDbClient,
    pub nonce_table_name: &'static str,
    pub mysql: Pool,
}
pub struct AuthOutsideCookie {
    pub domain: &'static str,
    pub cloudfront_key_pair_id: &'static str,
    pub cloudfront_resource: &'static str,
}
pub struct AuthOutsideSecret {
    pub ticket: AuthOutsideJwtSecret,
    pub api: AuthOutsideJwtSecret,
    pub cdn: AuthOutsideCdnSecret,
    pub reset_token: AuthOutsideJwtSecret,
}
pub struct AuthOutsideJwtSecret {
    pub decoding_key: DecodingKey<'static>,
    pub encoding_key: EncodingKey,
}
pub struct AuthOutsideCdnSecret {
    pub key: CloudfrontKey,
}
pub struct AuthOutsideEmail {
    pub ses: SesClient,
    pub ui_host: &'static str,
}
