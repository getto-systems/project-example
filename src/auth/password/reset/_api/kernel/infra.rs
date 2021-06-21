use serde::{Deserialize, Serialize};

use crate::auth::{
    auth_ticket::_api::kernel::data::ExpireDateTime, password::_api::kernel::data::ResetToken,
};

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
