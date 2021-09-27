use chrono::Duration;

use aws_cloudfront_cookie::CloudfrontKey;
use rusoto_core::Region;
use rusoto_dynamodb::DynamoDbClient;
use rusoto_ses::SesClient;
use sqlx::mysql::MySqlPoolOptions;

use crate::z_details::_common::jwt::helper::{decoding_key_from_ec_pem, encoding_key_from_ec_pem};

use crate::{
    auth::remote::x_outside_feature::{
        auth::feature::{
            AuthOutsideCloudfrontKey, AuthOutsideConfig, AuthOutsideEmail, AuthOutsideEncodingKey,
            AuthOutsideFeature, AuthOutsideResetTokenKey, AuthOutsideStore,
        },
        common::feature::AuthOutsideDecodingKey,
    },
    x_outside_feature::_auth::env::AuthEnv,
};

use crate::auth::ticket::remote::kernel::data::{ExpansionLimitDuration, ExpireDuration};

pub async fn new_auth_outside_feature(env: &'static AuthEnv) -> AuthOutsideFeature {
    AuthOutsideFeature {
        config: AuthOutsideConfig {
            // ticket の有効期限: 切れると再ログインが必要; renew で延長; 週末を挟めるように１桁日程度
            ticket_expires: ExpireDuration::with_duration(Duration::weeks(1)),
            // ticket の再延長期限: この期限を超えて renew できない
            ticket_expansion_limit: ExpansionLimitDuration::with_duration(Duration::weeks(12)),

            // api と cloudfront の有効期限: renew で再延長; 実用的な範囲の短い設定で１桁分程度
            api_expires: ExpireDuration::with_duration(Duration::minutes(5)),
            cloudfront_expires: ExpireDuration::with_duration(Duration::minutes(5)),

            // メールが届いてから作業が完了するまでの見込み時間
            reset_token_expires: ExpireDuration::with_duration(Duration::hours(8)),
        },
        store: AuthOutsideStore {
            dynamodb: DynamoDbClient::new(Region::ApNortheast1),
            nonce_table_name: &env.dynamodb_auth_nonce_table,
            mysql: MySqlPoolOptions::new()
                .max_connections(5)
                .connect(&env.mysql_auth_url)
                .await
                .expect("failed to connect mysql auth server"),
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
