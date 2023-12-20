use std::sync::Arc;

use aws_config::from_env;
use aws_sdk_dynamodb::Client as DynamoDbClient;
use aws_sdk_ses::Client as SesClient;
use chrono::Duration;
use tonic::Request;

use super::env::AuthEnv;

use crate::auth::x_outside_feature::feature::{
    AuthOutsideCloudfrontKey, AuthOutsideConfig, AuthOutsideDecodingKey, AuthOutsideEmail,
    AuthOutsideEncodingKey, AuthOutsideResetTokenKey, AuthOutsideStore,
};

use crate::common::api::jwt::helper::{decoding_key_from_ec_pem, encoding_key_from_ec_pem};

use crate::auth::data::{AwsCloudfrontKey, ExpansionLimitDuration, ExpireDuration};

pub struct AuthAppFeature {
    pub config: AuthOutsideConfig,
    pub store: AuthOutsideStore,
    pub decoding_key: AuthOutsideDecodingKey,
    pub encoding_key: AuthOutsideEncodingKey,
    pub cloudfront_key: Arc<AuthOutsideCloudfrontKey>,
    pub reset_token_key: AuthOutsideResetTokenKey,
    pub email: AuthOutsideEmail,
}

impl AuthAppFeature {
    pub async fn new(env: &'static AuthEnv) -> Self {
        let aws_config = from_env().region("ap-northeast-1").load().await;

        Self {
            config: AuthOutsideConfig {
                // ticket の有効期限: 切れると再ログインが必要; renew で延長; 週末を挟めるように１桁日程度
                authenticate_expires: ExpireDuration::with_duration(Duration::weeks(1)),
                // ticket の再延長期限: この期限を超えて renew できない
                authenticate_expansion_limit: ExpansionLimitDuration::with_duration(
                    Duration::weeks(12),
                ),

                // authorize と cdn の有効期限: renew で再延長; 実用的な範囲の短い設定で１桁分程度
                authorize_expires: ExpireDuration::with_duration(Duration::minutes(5)),
                cdn_expires: ExpireDuration::with_duration(Duration::minutes(5)),

                // メールが届いてからパスワードリセットが完了するまでの見込み時間
                reset_token_expires: ExpireDuration::with_duration(Duration::hours(8)),
            },
            store: AuthOutsideStore {
                dynamodb: Arc::new(DynamoDbClient::new(&aws_config)),
                ticket_table_name: &env.dynamodb_auth_ticket_table,
                user_table_name: &env.dynamodb_auth_user_table,
                login_id_table_name: &env.dynamodb_auth_login_id_table,
                reset_token_table_name: &env.dynamodb_auth_reset_token_table,
            },
            decoding_key: AuthOutsideDecodingKey {
                authenticate: Arc::new(decoding_key_from_ec_pem(&env.authenticate_public_key)),
                authorize: Arc::new(decoding_key_from_ec_pem(&env.authorize_public_key)),
            },
            encoding_key: AuthOutsideEncodingKey {
                authenticate: Arc::new(encoding_key_from_ec_pem(&env.authenticate_private_key)),
                authorize: Arc::new(encoding_key_from_ec_pem(&env.authorize_private_key)),
            },
            cloudfront_key: Arc::new(AuthOutsideCloudfrontKey {
                key: AwsCloudfrontKey::from_pem(&env.cloudfront_private_key)
                    .expect("failed to parse cloudfront private key"),
                key_pair_id: &env.cloudfront_key_pair_id,
                resource: &env.cloudfront_resource,
            }),
            reset_token_key: AuthOutsideResetTokenKey {
                decoding_key: Arc::new(decoding_key_from_ec_pem(&env.reset_token_public_key)),
                encoding_key: Arc::new(encoding_key_from_ec_pem(&env.reset_token_private_key)),
            },
            email: AuthOutsideEmail {
                ses: Arc::new(SesClient::new(&aws_config)),
                reset_password_url: &env.reset_password_url,
            },
        }
    }

    pub fn from_request<T>(request: &Request<T>) -> Arc<Self> {
        Arc::clone(
            request
                .extensions()
                .get::<Arc<Self>>()
                .expect("failed to get AppFeature"),
        )
    }
}
