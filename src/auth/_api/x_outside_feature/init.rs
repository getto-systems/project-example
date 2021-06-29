use std::collections::HashSet;

use chrono::Duration;

use aws_cloudfront_cookie::CloudfrontKey;

use crate::z_details::_api::jwt::helper::{decoding_key_from_ec_pem, encoding_key_from_ec_pem};

use crate::x_outside_feature::_api::env::Env;

use super::feature::{
    AuthOutsideCdnSecret, AuthOutsideConfig, AuthOutsideCookie, AuthOutsideEmail,
    AuthOutsideFeature, AuthOutsideJwtSecret, AuthOutsideSecret, AuthOutsideStore,
};

use crate::auth::{
    auth_ticket::_api::kernel::init::{MemoryAuthNonceMap, MemoryAuthTicketMap},
    auth_user::_api::kernel::init::MemoryAuthUserMap,
    password::{
        _api::kernel::init::MemoryAuthUserPasswordMap,
        reset::_api::request_token::init::MemoryResetTokenDestinationMap,
    },
};

use crate::auth::password::_api::kernel::infra::HashedPassword;

use crate::auth::{
    auth_ticket::_api::kernel::data::{ExpansionLimitDuration, ExpireDuration},
    auth_user::_api::kernel::data::{AuthUser, AuthUserExtract},
    login_id::_api::data::LoginId,
    password::reset::_api::request_token::data::{
        ResetTokenDestination, ResetTokenDestinationExtract,
    },
};

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
            // TODO それぞれ外部データベースを使うように
            nonce: MemoryAuthNonceMap::new().to_store(),
            ticket: MemoryAuthTicketMap::new().to_store(),
            user: MemoryAuthUserMap::with_user(admin_user()).to_store(),
            user_password: MemoryAuthUserPasswordMap::with_password(
                admin_login_id(),
                admin_user(),
                admin_password(),
            )
            .to_store(),
            reset_token_destination: MemoryResetTokenDestinationMap::with_destination(
                admin_login_id(),
                admin_reset_token_destination(),
            )
            .to_store(),
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
            ui_host: &env.ui_host,
        },
    }
}

fn admin_user() -> AuthUser {
    let mut granted_roles = HashSet::new();
    granted_roles.insert("admin".into());
    granted_roles.insert("dev-docs".into());

    AuthUserExtract {
        user_id: "admin".into(),
        granted_roles,
    }
    .into()
}
fn admin_login_id() -> LoginId {
    LoginId::validate("admin".into()).unwrap()
}
fn admin_password() -> HashedPassword {
    HashedPassword::new("$argon2id$v=19$m=4096,t=3,p=1$wL7bldJ+qUCSNYyrgm6OUA$BW+HlZoe6tYaO4yZ3PwQ+F/hj640LiKtfuM8B6YZ+bk".into())
}
fn admin_reset_token_destination() -> ResetTokenDestination {
    ResetTokenDestinationExtract {
        email: "shun@getto.systems".into(),
    }
    .into()
}
