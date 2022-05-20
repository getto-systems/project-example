use std::sync::Arc;

use chrono::Duration;
use rusoto_core::Region;
use rusoto_dynamodb::DynamoDbClient;
use rusoto_ses::SesClient;
use tonic::{metadata::MetadataMap, Request};

use aws_cloudfront_cookie::CloudfrontKey;

use super::env::AuthEnv;

use crate::auth::x_outside_feature::feature::{
    AuthOutsideCloudfrontKey, AuthOutsideConfig, AuthOutsideDecodingKey, AuthOutsideEmail,
    AuthOutsideEncodingKey, AuthOutsideResetTokenKey, AuthOutsideStore,
};

use crate::z_lib::jwt::helper::{decoding_key_from_ec_pem, encoding_key_from_ec_pem};

use crate::auth::data::{ExpansionLimitDuration, ExpireDuration};

pub struct AuthAppFeature {
    pub config: AuthOutsideConfig,
    pub store: AuthOutsideStore,
    pub decoding_key: AuthOutsideDecodingKey,
    pub encoding_key: AuthOutsideEncodingKey,
    pub cloudfront_key: AuthOutsideCloudfrontKey,
    pub reset_token_key: AuthOutsideResetTokenKey,
    pub email: AuthOutsideEmail,
}

impl AuthAppFeature {
    pub fn new(env: &'static AuthEnv) -> Self {
        Self {
            config: AuthOutsideConfig {
                // ticket の有効期限: 切れると再ログインが必要; renew で延長; 週末を挟めるように１桁日程度
                ticket_expires: ExpireDuration::with_duration(Duration::weeks(1)),
                // ticket の再延長期限: この期限を超えて renew できない
                ticket_expansion_limit: ExpansionLimitDuration::with_duration(Duration::weeks(12)),

                // api と cloudfront の有効期限: renew で再延長; 実用的な範囲の短い設定で１桁分程度
                api_expires: ExpireDuration::with_duration(Duration::minutes(5)),
                cloudfront_expires: ExpireDuration::with_duration(Duration::minutes(5)),

                // メールが届いてからパスワードリセットが完了するまでの見込み時間
                reset_token_expires: ExpireDuration::with_duration(Duration::hours(8)),
            },
            store: AuthOutsideStore {
                dynamodb: DynamoDbClient::new(Region::ApNortheast1),
                nonce_table_name: &env.dynamodb_auth_nonce_table,
                ticket_table_name: &env.dynamodb_auth_ticket_table,
                user_table_name: &env.dynamodb_auth_user_table,
                login_id_table_name: &env.dynamodb_auth_login_id_table,
                reset_token_table_name: &env.dynamodb_auth_reset_token_table,
            },
            decoding_key: AuthOutsideDecodingKey {
                ticket: decoding_key_from_ec_pem(&env.ticket_public_key),
                api: decoding_key_from_ec_pem(&env.api_public_key),
            },
            encoding_key: AuthOutsideEncodingKey {
                ticket: encoding_key_from_ec_pem(&env.ticket_private_key),
                api: encoding_key_from_ec_pem(&env.api_private_key),
            },
            cloudfront_key: AuthOutsideCloudfrontKey {
                key: CloudfrontKey::from_pem(&env.cloudfront_private_key)
                    .expect("failed to parse cloudfront private key"),
                key_pair_id: &env.cloudfront_key_pair_id,
                resource: &env.cloudfront_resource,
            },
            reset_token_key: AuthOutsideResetTokenKey {
                decoding_key: decoding_key_from_ec_pem(&env.reset_token_public_key),
                encoding_key: encoding_key_from_ec_pem(&env.reset_token_private_key),
            },
            email: AuthOutsideEmail {
                ses: SesClient::new(Region::ApNortheast1),
                reset_password_url: &env.reset_password_url,
            },
        }
    }
}

pub struct AuthTonicRequest<T> {
    pub feature: Arc<AuthAppFeature>,
    pub metadata: MetadataMap,
    pub request: T,
}

pub fn extract_auth_request<T>(request: Request<T>) -> AuthTonicRequest<T> {
    let feature = request
        .extensions()
        .get::<Arc<AuthAppFeature>>()
        .expect("failed to get AppFeature")
        .clone();

    AuthTonicRequest {
        feature,
        // metadata と inner の両方を into してくれるやつが無いため、to_owned する
        metadata: request.metadata().to_owned(),
        request: request.into_inner(),
    }
}
