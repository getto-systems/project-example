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

#[cfg(test)]
pub mod test {
    use chrono::DateTime;
    use chrono::Utc;

    use super::super::AuthClock;

    use super::super::super::data::AuthDateTime;

    #[derive(Clone)]
    pub struct StaticChronoAuthClock {
        now: DateTime<Utc>,
    }

    impl StaticChronoAuthClock {
        pub const fn new(now: DateTime<Utc>) -> Self {
            Self { now }
        }
    }

    impl AuthClock for StaticChronoAuthClock {
        fn now(&self) -> AuthDateTime {
            AuthDateTime::from_now(self.now.clone())
        }
    }
}
