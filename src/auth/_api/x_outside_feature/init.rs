use chrono::Duration;

use aws_cloudfront_cookie::CloudfrontKey;

use super::feature::{
    AuthOutsideCdnSecret, AuthOutsideConfig, AuthOutsideCookie, AuthOutsideFeature,
    AuthOutsideJwtSecret, AuthOutsideSecret, AuthOutsideStore,
};
use crate::x_outside_feature::_api::{env::Env, secret::Secret};

use crate::auth::auth_ticket::_api::{
    encode::infra::token_encoder::JwtTokenEncoderKey,
    kernel::infra::{
        nonce_repository::MemoryAuthNonceRepository, ticket_repository::MemoryAuthTicketRepository,
    },
    validate::infra::token_validator::JwtTokenValidatorKey,
};
use crate::auth::auth_user::_api::kernel::infra::user_repository::MemoryAuthUserRepository;
use crate::auth::password::_api::authenticate::infra::password_repository::MemoryAuthUserPasswordRepository;

use crate::auth::auth_ticket::_api::kernel::data::{ExpansionLimitDuration, ExpireDuration};

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
        },
        store: AuthOutsideStore {
            // TODO それぞれ外部データベースを使うように
            nonce: MemoryAuthNonceRepository::new_store(),
            ticket: MemoryAuthTicketRepository::new_store(),
            user: MemoryAuthUserRepository::new_store(),
            user_password: MemoryAuthUserPasswordRepository::new_store(),
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
        },
    }
}
