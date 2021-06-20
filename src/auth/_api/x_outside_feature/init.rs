use std::collections::HashSet;

use chrono::Duration;

use aws_cloudfront_cookie::CloudfrontKey;

use crate::auth::password::reset::_api::request_token::infra::destination_repository::MemoryResetTokenDestinationMap;
use crate::z_details::_api::jwt::helper::JwtTokenEncoderKey;

use super::feature::{
    AuthOutsideCdnSecret, AuthOutsideConfig, AuthOutsideCookie, AuthOutsideEmail,
    AuthOutsideFeature, AuthOutsideJwtSecret, AuthOutsideSecret, AuthOutsideStore,
};
use crate::x_outside_feature::_api::{env::Env, secret::Secret};

use crate::auth::{
    auth_ticket::_api::{
        kernel::infra::{
            nonce_repository::MemoryAuthNonceMap, ticket_repository::MemoryAuthTicketMap,
        },
        validate::infra::token_validator::JwtTokenValidatorKey,
    },
    auth_user::_api::kernel::infra::user_repository::MemoryAuthUserMap,
    password::{
        _api::authenticate::infra::{
            password_repository::MemoryAuthUserPasswordMap, HashedPassword,
        },
        reset::_api::kernel::infra::token_repository::MemoryResetTokenMap,
    },
};

use crate::auth::{
    auth_ticket::_api::kernel::data::{ExpansionLimitDuration, ExpireDuration},
    auth_user::_api::kernel::data::{AuthUser, AuthUserExtract},
    login_id::_api::data::LoginId,
};

pub fn new_auth_outside_feature(env: &Env, secret: &impl Secret) -> AuthOutsideFeature {
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
            reset_token_expires: ExpireDuration::with_duration(Duration::hours(3)),
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
            reset_token: MemoryResetTokenMap::new().to_store(),
            reset_token_destination: MemoryResetTokenDestinationMap::new().to_store(),
        },
        cookie: AuthOutsideCookie {
            domain: env.load("DOMAIN"),
            cloudfront_key_pair_id: secret.load("CLOUDFRONT_KEY_PAIR_ID"),
            cloudfront_resource: env.load("CLOUDFRONT_RESOURCE"),
        },
        secret: AuthOutsideSecret {
            ticket: AuthOutsideJwtSecret {
                decoding_key: JwtTokenValidatorKey::Ec(secret.load("TICKET_PUBLIC_KEY")),
                encoding_key: JwtTokenEncoderKey::ec(secret.load("TICKET_PRIVATE_KEY")),
            },
            api: AuthOutsideJwtSecret {
                decoding_key: JwtTokenValidatorKey::Ec(secret.load("API_PUBLIC_KEY")),
                encoding_key: JwtTokenEncoderKey::ec(secret.load("API_PRIVATE_KEY")),
            },
            cdn: AuthOutsideCdnSecret {
                key: CloudfrontKey::from_pem(secret.load("CLOUDFRONT_PRIVATE_KEY"))
                    .expect("failed to parse cloudfront private key"),
            },
            reset_token: AuthOutsideJwtSecret {
                decoding_key: JwtTokenValidatorKey::Ec(secret.load("RESET_TOKEN_PUBLIC_KEY")),
                encoding_key: JwtTokenEncoderKey::ec(secret.load("RESET_TOKEN_PRIVATE_KEY")),
            },
        },
        email: AuthOutsideEmail {
            ui_host: env.load("UI_HOST"),
            sender_address: "labo@message.getto.systems".into(),
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
    LoginId::validate("admin".to_string()).unwrap()
}
fn admin_password() -> HashedPassword {
    HashedPassword::new("$argon2id$v=19$m=4096,t=3,p=1$wL7bldJ+qUCSNYyrgm6OUA$BW+HlZoe6tYaO4yZ3PwQ+F/hj640LiKtfuM8B6YZ+bk".into())
}
