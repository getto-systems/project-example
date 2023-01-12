use chrono::{DateTime, Duration, NaiveDateTime, Utc};

#[derive(Clone)]
pub struct AuthDateTime(DateTime<Utc>);

impl AuthDateTime {
    pub(in crate::auth) const fn restore(now: DateTime<Utc>) -> Self {
        Self(now)
    }

    pub const fn extract(self) -> DateTime<Utc> {
        self.0
    }

    pub fn expires(&self, duration: &ExpireDuration) -> ExpireDateTime {
        ExpireDateTime(self.0 + duration.0)
    }

    pub fn expansion_limit(&self, duration: &ExpansionLimitDuration) -> ExpansionLimitDateTime {
        ExpansionLimitDateTime(self.0 + duration.0)
    }

    pub fn expires_with_limit(
        self,
        duration: &ExpireDuration,
        limit: &ExpansionLimitDateTime,
    ) -> ExpireDateTime {
        let expires = self.0 + duration.0;
        if expires > limit.0 {
            ExpireDateTime(limit.0.clone())
        } else {
            ExpireDateTime(expires)
        }
    }
}

#[derive(Clone)]
pub struct ExpireDateTime(DateTime<Utc>);

impl ExpireDateTime {
    pub(in crate::auth) const fn restore(value: DateTime<Utc>) -> Self {
        Self(value)
    }

    pub(in crate::auth) fn restore_from_timestamp(value: i64) -> Self {
        Self::restore(DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp_opt(value, 0).unwrap_or_default(),
            Utc,
        ))
    }

    pub const fn extract(self) -> DateTime<Utc> {
        self.0
    }

    pub fn extract_to_timestamp(self) -> i64 {
        self.0.timestamp()
    }

    pub fn has_elapsed(&self, now: &AuthDateTime) -> bool {
        self.0 < now.0
    }
}

impl std::fmt::Display for ExpireDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
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

    pub const fn extract(self) -> DateTime<Utc> {
        self.0
    }

    pub fn has_elapsed(&self, now: &AuthDateTime) -> bool {
        self.0 < now.0
    }
}

impl std::fmt::Display for ExpansionLimitDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
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
