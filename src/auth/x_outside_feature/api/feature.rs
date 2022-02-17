use jsonwebtoken::{DecodingKey, EncodingKey};
use rusoto_dynamodb::DynamoDbClient;
use rusoto_ses::SesClient;

use aws_cloudfront_cookie::CloudfrontKey;

use crate::{
    auth::ticket::kernel::data::{ExpansionLimitDuration, ExpireDuration},
    z_lib::service::x_outside_feature::feature::GoogleServiceAuthorizerOutsideFeature,
};

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
    pub reset_token_table_name: &'static str,
}
pub struct AuthOutsideEncodingKey {
    pub ticket: EncodingKey,
    pub api: EncodingKey,
}
pub struct AuthOutsideDecodingKey {
    pub ticket: DecodingKey,
    pub api: DecodingKey,
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

pub struct AuthProxyOutsideFeature {
    pub service: AuthOutsideService,
    pub decoding_key: AuthOutsideDecodingKey,
    pub cookie: AuthOutsideCookie,
}
pub struct AuthOutsideService {
    pub service_url: &'static str,
    pub google_authorizer: GoogleServiceAuthorizerOutsideFeature,
}
pub struct AuthOutsideCookie {
    pub domain: &'static str,
    pub cloudfront_key_pair_id: &'static str,
}
