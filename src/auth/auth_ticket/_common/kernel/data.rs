use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

use chrono::{DateTime, Duration, Utc};

#[derive(Clone)]
pub struct AuthNonceValue(String);

impl AuthNonceValue {
    pub const fn new(nonce: String) -> Self {
        Self(nonce)
    }

    pub fn extract(self) -> String {
        self.0
    }

    #[cfg(test)]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Clone)]
pub struct AuthToken {
    token: AuthTokenValue,
    expires: ExpireDateTime,
}

#[derive(Clone)]
pub struct AuthTokenExtract {
    pub token: String,
    pub expires: i64,
}

#[derive(Clone)]
pub struct AuthTokenValue(String);

impl AuthTokenValue {
    pub const fn new(token: String) -> Self {
        Self(token)
    }

    pub fn extract(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

pub struct AuthTokenEncoded {
    pub ticket_token: AuthTokenExtract,
    pub api_token: AuthTokenExtract,
    pub cloudfront_tokens: HashMap<CloudfrontTokenKind, AuthTokenExtract>,
}

#[derive(Eq, PartialEq, Hash)]
pub enum CloudfrontTokenKind {
    KeyPairId,
    Policy,
    Signature,
}

#[derive(Clone)]
pub struct AuthDateTime(DateTime<Utc>);

impl AuthDateTime {
    pub(in crate::auth) const fn restore(now: DateTime<Utc>) -> Self {
        Self(now)
    }

    pub fn extract(self) -> DateTime<Utc> {
        self.0
    }

    pub fn expires(self, duration: &ExpireDuration) -> ExpireDateTime {
        ExpireDateTime(self.0 + duration.0)
    }

    pub fn expansion_limit(self, duration: &ExpansionLimitDuration) -> ExpansionLimitDateTime {
        ExpansionLimitDateTime(self.0 + duration.0)
    }

    pub fn expires_with_limit(
        self,
        duration: &ExpireDuration,
        limit: ExpansionLimitDateTime,
    ) -> ExpireDateTime {
        let expires = self.0 + duration.0;
        if expires > limit.0 {
            ExpireDateTime(limit.0)
        } else {
            ExpireDateTime(expires)
        }
    }
}

#[derive(Clone)]
pub struct ExpireDateTime(DateTime<Utc>);

impl ExpireDateTime {
    pub(in crate::auth) const fn restore(time: DateTime<Utc>) -> Self {
        Self(time)
    }

    pub fn has_elapsed(&self, now: &AuthDateTime) -> bool {
        self.0 < now.0
    }

    pub fn extract(self) -> DateTime<Utc> {
        self.0
    }
}

impl Display for ExpireDateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.0.fmt(f)
    }
}

#[derive(Clone, Copy)]
pub struct ExpireDuration(Duration);

impl ExpireDuration {
    pub fn with_duration(duration: Duration) -> Self {
        Self(duration)
    }
}

#[derive(Clone)]
pub struct ExpansionLimitDateTime(DateTime<Utc>);

impl ExpansionLimitDateTime {
    pub(in crate::auth) const fn restore(time: DateTime<Utc>) -> Self {
        Self(time)
    }

    pub fn extract(self) -> DateTime<Utc> {
        self.0
    }
}

impl Display for ExpansionLimitDateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.0.fmt(f)
    }
}

#[derive(Clone, Copy)]
pub struct ExpansionLimitDuration(Duration);

impl ExpansionLimitDuration {
    pub fn with_duration(duration: Duration) -> Self {
        Self(duration)
    }
}
