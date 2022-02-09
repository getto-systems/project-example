use chrono::Utc;

use crate::auth::ticket::kernel::infra::AuthClock;

use crate::auth::ticket::kernel::data::AuthDateTime;

pub struct ChronoAuthClock;

impl ChronoAuthClock {
    pub const fn new() -> Self {
        Self {}
    }
}

impl AuthClock for ChronoAuthClock {
    fn now(&self) -> AuthDateTime {
        AuthDateTime::restore(Utc::now())
    }
}

#[cfg(test)]
pub mod test {
    use chrono::{DateTime, Utc};

    use crate::auth::ticket::kernel::infra::AuthClock;

    use crate::auth::ticket::kernel::data::AuthDateTime;

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
            AuthDateTime::restore(self.now.clone())
        }
    }
}