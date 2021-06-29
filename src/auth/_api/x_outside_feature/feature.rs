use aws_cloudfront_cookie::CloudfrontKey;
use jsonwebtoken::{DecodingKey, EncodingKey};

use crate::auth::{
    auth_ticket::_api::kernel::init::{MemoryAuthNonceStore, MemoryAuthTicketStore},
    auth_user::_api::kernel::init::MemoryAuthUserStore,
    password::_api::kernel::init::MemoryAuthUserPasswordStore,
};

use crate::auth::password::reset::_api::request_token::infra::destination_repository::MemoryResetTokenDestinationStore;

use crate::auth::auth_ticket::_api::kernel::data::{ExpansionLimitDuration, ExpireDuration};

pub struct AuthOutsideFeature {
    pub config: AuthOutsideConfig,
    pub store: AuthOutsideStore,
    pub cookie: AuthOutsideCookie,
    pub secret: AuthOutsideSecret,
    pub email: AuthOutsideEmail,
}
pub struct AuthOutsideConfig {
    pub ticket_expires: ExpireDuration,
    pub ticket_expansion_limit: ExpansionLimitDuration,
    pub api_expires: ExpireDuration,
    pub cdn_expires: ExpireDuration,
    pub reset_token_expires: ExpireDuration,
}
pub struct AuthOutsideStore {
    pub nonce: MemoryAuthNonceStore,
    pub ticket: MemoryAuthTicketStore,
    pub user: MemoryAuthUserStore,
    pub user_password: MemoryAuthUserPasswordStore,
    pub reset_token_destination: MemoryResetTokenDestinationStore,
}
pub struct AuthOutsideCookie {
    pub domain: &'static str,
    pub cloudfront_key_pair_id: &'static str,
    pub cloudfront_resource: &'static str,
}
pub struct AuthOutsideSecret {
    pub ticket: AuthOutsideJwtSecret,
    pub api: AuthOutsideJwtSecret,
    pub cdn: AuthOutsideCdnSecret,
    pub reset_token: AuthOutsideJwtSecret,
}
pub struct AuthOutsideJwtSecret {
    pub decoding_key: DecodingKey<'static>,
    pub encoding_key: EncodingKey,
}
pub struct AuthOutsideCdnSecret {
    pub key: CloudfrontKey,
}
pub struct AuthOutsideEmail {
    pub ui_host: &'static str,
}
