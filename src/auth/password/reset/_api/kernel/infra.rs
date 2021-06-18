pub mod token_repository;

use serde::{Deserialize, Serialize};

use super::data::ResetToken;
use crate::{
    auth::{
        auth_ticket::_api::kernel::data::{AuthDateTime, ExpireDateTime},
        password::reset::_api::request_token::data::ResetTokenDestination,
    },
    z_details::_api::repository::data::RepositoryError,
};

pub trait ResetTokenRepository {
    fn register(
        &self,
        destination: ResetTokenDestination,
        token_generator: &impl ResetTokenGenerator,
        expires: ExpireDateTime,
        registered_at: AuthDateTime,
    ) -> Result<ResetToken, RepositoryError>;

    fn get(&self, token: &ResetToken) -> Result<Option<ResetTokenEntry>, RepositoryError>;

    fn discard(&self, token: ResetToken, discard_at: AuthDateTime) -> Result<(), RepositoryError>;
}

#[derive(Clone)]
pub enum ResetTokenEntry {
    Active(ResetTokenActiveEntry),
    Discarded,
}

#[derive(Clone)]
pub struct ResetTokenActiveEntry {
    destination: ResetTokenDestination,
    expires: ExpireDateTime,
}

impl ResetTokenActiveEntry {
    pub const fn new(destination: ResetTokenDestination, expires: ExpireDateTime) -> Self {
        Self {
            destination,
            expires,
        }
    }

    pub fn has_expired(&self, now: AuthDateTime) -> bool {
        self.expires.has_elapsed(now)
    }
}

pub trait ResetTokenGenerator {
    fn generate(&self) -> ResetToken;
}

#[derive(Serialize, Deserialize)]
pub struct ResetTokenJwtClaims {
    sub: String,
    exp: i64,
}

impl ResetTokenJwtClaims {
    pub fn from_token(token: ResetToken, expires: ExpireDateTime) -> Self {
        Self {
            sub: token.extract(),
            exp: expires.timestamp(),
        }
    }
}

impl Into<ResetToken> for ResetTokenJwtClaims {
    fn into(self) -> ResetToken {
        ResetToken::new(self.sub)
    }
}
