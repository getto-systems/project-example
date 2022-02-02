use chrono::{Utc, DateTime};
use serde::{Deserialize, Serialize};

use crate::auth::{
    ticket::kernel::remote::data::{AuthDateTime, ExpireDateTime},
    user::{
        login_id::kernel::data::LoginId,
        password::reset::remote::kernel::data::{ResetToken, ResetTokenDestination, ResetTokenDestinationExtract},
    },
};

pub struct ResetTokenEntry {
    login_id: LoginId,
    destination: ResetTokenDestination,
    expires: ExpireDateTime,
    reset_at: Option<AuthDateTime>,
}

impl ResetTokenEntry {
    pub fn verify_login_id(&self, login_id: &LoginId) -> bool {
        self.login_id.as_str() == login_id.as_str()
    }

    pub fn has_expired(&self, now: &AuthDateTime) -> bool {
        self.expires.has_elapsed(now)
    }

    pub fn has_already_reset(&self) -> bool {
        self.reset_at.is_some()
    }

    pub fn into_destination(self) -> ResetTokenDestination {
        self.destination
    }
}

pub struct ResetTokenEntryExtract {
    pub login_id: String,
    pub destination: ResetTokenDestinationExtract,
    pub expires: DateTime<Utc>,
    pub reset_at: Option<DateTime<Utc>>,
}

impl ResetTokenEntryExtract {
    pub(in crate::auth) fn restore(self) -> ResetTokenEntry {
        ResetTokenEntry {
            login_id: LoginId::restore(self.login_id),
            destination: self.destination.restore(),
            expires: ExpireDateTime::restore(self.expires),
            reset_at: self.reset_at.map(AuthDateTime::restore),
        }
    }
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
            exp: expires.extract().timestamp(),
        }
    }
}

impl Into<ResetToken> for ResetTokenJwtClaims {
    fn into(self) -> ResetToken {
        ResetToken::new(self.sub)
    }
}
