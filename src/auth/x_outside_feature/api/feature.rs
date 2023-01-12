use jsonwebtoken::{DecodingKey, EncodingKey};
use rusoto_dynamodb::DynamoDbClient;
use rusoto_ses::SesClient;

use aws_cloudfront_cookie::CloudfrontKey;

use crate::{
    auth::kernel::data::{ExpansionLimitDuration, ExpireDuration},
    common::x_outside_feature::feature::CommonOutsideService,
};

pub struct AuthOutsideConfig {
    pub authenticate_expires: ExpireDuration,
    pub authenticate_expansion_limit: ExpansionLimitDuration,
    pub authorize_expires: ExpireDuration,
    pub cdn_expires: ExpireDuration,
    pub reset_token_expires: ExpireDuration,
}
pub struct AuthOutsideStore {
    pub dynamodb: DynamoDbClient,
    pub ticket_table_name: &'static str,
    pub user_table_name: &'static str,
    pub login_id_table_name: &'static str,
    pub reset_token_table_name: &'static str,
}
pub struct AuthOutsideEncodingKey {
    pub authenticate: EncodingKey,
    pub authorize: EncodingKey,
}
pub struct AuthOutsideDecodingKey {
    pub authenticate: DecodingKey,
    pub authorize: DecodingKey,
}
pub struct AuthOutsideAuthorizeKey {
    pub key: DecodingKey,
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

pub struct AuthServiceOutsideFeature {
    pub service: CommonOutsideService,
    pub decoding_key: AuthOutsideAuthorizeKey,
}
pub struct AuthProxyOutsideFeature {
    pub service: CommonOutsideService,
    pub cookie: AuthOutsideCookie,
    pub decoding_key: AuthOutsideDecodingKey,
}
pub struct AuthOutsideCookie {
    pub domain: &'static str,
    pub cloudfront_key_pair_id: &'static str,
}
