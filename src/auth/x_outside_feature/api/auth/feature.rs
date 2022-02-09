use jsonwebtoken::{DecodingKey, EncodingKey};
use rusoto_dynamodb::DynamoDbClient;
use rusoto_ses::SesClient;

use aws_cloudfront_cookie::CloudfrontKey;

use crate::auth::x_outside_feature::common::feature::AuthOutsideDecodingKey;

use crate::auth::ticket::kernel::data::{ExpansionLimitDuration, ExpireDuration};

pub struct AuthOutsideFeature {
    pub(in crate::auth) config: AuthOutsideConfig,
    pub(in crate::auth) store: AuthOutsideStore,
    pub decoding_key: AuthOutsideDecodingKey,
    pub(in crate::auth) encoding_key: AuthOutsideEncodingKey,
    pub(in crate::auth) cloudfront_key: AuthOutsideCloudfrontKey,
    pub(in crate::auth) reset_token_key: AuthOutsideResetTokenKey,
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
    pub ticket_table_name: &'static str,
    pub user_table_name: &'static str,
    pub login_id_table_name: &'static str,
    pub destination_table_name: &'static str,
    pub reset_token_table_name: &'static str,
}
pub struct AuthOutsideEncodingKey {
    pub ticket: EncodingKey,
    pub api: EncodingKey,
}
pub struct AuthOutsideResetTokenKey {
    pub decoding_key: DecodingKey,
    pub encoding_key: EncodingKey,
}
pub struct AuthOutsideCloudfrontKey {
    pub key: CloudfrontKey,
    pub key_pair_id: &'static str,
    pub resource: &'static str,
}
pub struct AuthOutsideEmail {
    pub ses: SesClient,
    pub reset_password_url: &'static str,
}
