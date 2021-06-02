use chrono::Utc;

use super::AuthClock;

use super::super::data::AuthDateTime;

pub struct ChronoAuthClock;

impl ChronoAuthClock {
    pub const fn new() -> Self {
        Self {}
    }
}

impl AuthClock for ChronoAuthClock {
    fn now(&self) -> AuthDateTime {
        AuthDateTime::from_now(Utc::now())
    }
}
