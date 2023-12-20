use std::sync::{Arc, Mutex};

use aws_sdk_dynamodb::Client as DynamoDbClient;
use aws_sdk_ses::Client as SesClient;
use jsonwebtoken::{DecodingKey, EncodingKey};

use crate::common::api::service::detail::authorizer::GoogleServiceAuthorizerToken;

use crate::auth::{
    kernel::data::{ExpansionLimitDuration, ExpireDuration},
    ticket::kernel::aws::cloudfront::data::AwsCloudfrontKey,
};

pub struct AuthOutsideConfig {
    pub authenticate_expires: ExpireDuration,
    pub authenticate_expansion_limit: ExpansionLimitDuration,
    pub authorize_expires: ExpireDuration,
    pub cdn_expires: ExpireDuration,
    pub reset_token_expires: ExpireDuration,
}
pub struct AuthOutsideStore {
    pub dynamodb: Arc<DynamoDbClient>,
    pub ticket_table_name: &'static str,
    pub user_table_name: &'static str,
    pub login_id_table_name: &'static str,
    pub reset_token_table_name: &'static str,
}
pub struct AuthOutsideEncodingKey {
    pub authenticate: Arc<EncodingKey>,
    pub authorize: Arc<EncodingKey>,
}
pub struct AuthOutsideDecodingKey {
    pub authenticate: Arc<DecodingKey>,
    pub authorize: Arc<DecodingKey>,
}
pub struct AuthOutsideAuthorizeKey {
    pub key: Arc<DecodingKey>,
}
pub struct AuthOutsideResetTokenKey {
    pub decoding_key: Arc<DecodingKey>,
    pub encoding_key: Arc<EncodingKey>,
}
pub struct AuthOutsideCloudfrontKey {
    pub key: AwsCloudfrontKey,
    pub key_pair_id: &'static str,
    pub resource: &'static str,
}
pub struct AuthOutsideEmail {
    pub ses: Arc<SesClient>,
    pub reset_password_url: &'static str,
}

pub struct AuthServiceOutsideFeature {
    pub service_url: &'static str,
    pub google_authorize_store: Arc<Mutex<GoogleServiceAuthorizerToken>>,
    pub decoding_key: AuthOutsideAuthorizeKey,
}
pub struct AuthProxyOutsideFeature {
    pub service_url: &'static str,
    pub google_authorize_store: Arc<Mutex<GoogleServiceAuthorizerToken>>,
    pub cookie: AuthOutsideCookie,
    pub decoding_key: AuthOutsideDecodingKey,
}
pub struct AuthOutsideCookie {
    pub domain: &'static str,
    pub cloudfront_key_pair_id: &'static str,
}
