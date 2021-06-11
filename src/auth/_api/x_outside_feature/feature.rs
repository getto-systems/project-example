use std::sync::Mutex;

use aws_cloudfront_cookie::CloudfrontKey;
use jsonwebtoken::EncodingKey;

use crate::auth::auth_ticket::_api::{
    kernel::infra::{
        nonce_repository::MemoryAuthNonceMap, ticket_repository::MemoryAuthTicketMap,
    },
    validate::infra::token_validator::JwtTokenValidatorKey,
};
use crate::auth::auth_user::_api::kernel::infra::user_repository::MemoryAuthUserStore;
use crate::auth::password::_api::authenticate::infra::password_repository::MemoryAuthUserPasswordStore;

use crate::auth::auth_ticket::_api::kernel::data::{ExpansionLimitDuration, ExpireDuration};

pub struct AuthOutsideFeature {
    pub config: AuthOutsideConfig,
    pub store: AuthOutsideStore,
    pub cookie: AuthOutsideCookie,
    pub secret: AuthOutsideSecret,
}
pub struct AuthOutsideConfig {
    pub ticket_expires: ExpireDuration,
    pub ticket_expansion_limit: ExpansionLimitDuration,
    pub api_expires: ExpireDuration,
    pub cdn_expires: ExpireDuration,
}
pub struct AuthOutsideStore {
    pub nonce: Mutex<MemoryAuthNonceMap>,
    pub ticket: Mutex<MemoryAuthTicketMap>,
    pub user: MemoryAuthUserStore,
    pub user_password: MemoryAuthUserPasswordStore,
}
pub struct AuthOutsideCookie {
    pub domain: String,
    pub cloudfront_key_pair_id: String,
    pub cloudfront_resource: String,
}
pub struct AuthOutsideSecret {
    pub ticket: AuthOutsideJwtSecret,
    pub api: AuthOutsideJwtSecret,
    pub cdn: AuthOutsideCdnSecret,
}
pub struct AuthOutsideJwtSecret {
    pub decoding_key: JwtTokenValidatorKey,
    pub encoding_key: EncodingKey,
}
pub struct AuthOutsideCdnSecret {
    pub key: CloudfrontKey,
}
