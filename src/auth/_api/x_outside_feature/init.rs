use chrono::Duration;

use aws_cloudfront_cookie::CloudfrontKey;
use mysql::{Opts, Pool};
use rusoto_core::Region;
use rusoto_dynamodb::DynamoDbClient;
use rusoto_ses::SesClient;

use crate::z_details::_api::jwt::helper::{decoding_key_from_ec_pem, encoding_key_from_ec_pem};

use crate::x_outside_feature::_api::env::Env;

use super::feature::{
    AuthOutsideCdnSecret, AuthOutsideConfig, AuthOutsideCookie, AuthOutsideEmail,
    AuthOutsideFeature, AuthOutsideJwtSecret, AuthOutsideSecret, AuthOutsideStore,
};

use crate::auth::auth_ticket::_api::kernel::data::{ExpansionLimitDuration, ExpireDuration};

pub fn new_auth_outside_feature(env: &'static Env) -> AuthOutsideFeature {
    AuthOutsideFeature {
        config: AuthOutsideConfig {
            // ticket の有効期限: 切れると再ログインが必要; renew で延長; 週末を挟めるように１桁日程度
            ticket_expires: ExpireDuration::with_duration(Duration::weeks(1)),
            // ticket の再延長期限: この期限を超えて renew できない
            ticket_expansion_limit: ExpansionLimitDuration::with_duration(Duration::weeks(12)),

            // api と cdn の有効期限: renew で再延長; 実用的な範囲の短い設定で１桁分程度
            api_expires: ExpireDuration::with_duration(Duration::minutes(5)),
            cdn_expires: ExpireDuration::with_duration(Duration::minutes(5)),

            // メールが届いてから作業が完了するまでの見込み時間
            reset_token_expires: ExpireDuration::with_duration(Duration::hours(8)),
        },
        store: AuthOutsideStore {
            dynamodb: DynamoDbClient::new(Region::ApNortheast1),
            nonce_table_name: &env.dynamodb_auth_nonce_table,
            mysql: Pool::new(
                Opts::from_url(&env.mysql_auth_url).expect("failed to parse connection url".into()),
            )
            .expect("failed to connect mysql".into()),
        },
        cookie: AuthOutsideCookie {
            domain: &env.domain,
            cloudfront_key_pair_id: &env.cloudfront_key_pair_id,
            cloudfront_resource: &env.cloudfront_resource,
        },
        secret: AuthOutsideSecret {
            ticket: AuthOutsideJwtSecret {
                decoding_key: decoding_key_from_ec_pem(&env.ticket_public_key),
                encoding_key: encoding_key_from_ec_pem(&env.ticket_private_key),
            },
            api: AuthOutsideJwtSecret {
                decoding_key: decoding_key_from_ec_pem(&env.api_public_key),
                encoding_key: encoding_key_from_ec_pem(&env.api_private_key),
            },
            cdn: AuthOutsideCdnSecret {
                key: CloudfrontKey::from_pem(&env.cloudfront_private_key)
                    .expect("failed to parse cloudfront private key"),
            },
            reset_token: AuthOutsideJwtSecret {
                decoding_key: decoding_key_from_ec_pem(&env.reset_token_public_key),
                encoding_key: encoding_key_from_ec_pem(&env.reset_token_private_key),
            },
        },
        email: AuthOutsideEmail {
            ses: SesClient::new(Region::ApNortheast1),
            ui_host: &env.ui_host,
        },
    }
}
