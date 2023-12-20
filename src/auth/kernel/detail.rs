use chrono::Utc;

use crate::auth::kernel::infra::AuthClock;

use crate::auth::kernel::data::AuthDateTime;

pub struct ChronoAuthClock;

impl AuthClock for ChronoAuthClock {
    fn now(&self) -> AuthDateTime {
        AuthDateTime::restore(Utc::now())
    }
}

#[cfg(test)]
pub mod test {
    use chrono::{DateTime, Utc};

    use crate::auth::kernel::infra::AuthClock;

    use crate::auth::kernel::data::AuthDateTime;

    #[derive(Clone)]
    pub struct MockChronoAuthClock {
        now: DateTime<Utc>,
    }

    impl MockChronoAuthClock {
        pub const fn new(now: DateTime<Utc>) -> Self {
            Self { now }
        }
    }

    impl AuthClock for MockChronoAuthClock {
        fn now(&self) -> AuthDateTime {
            AuthDateTime::restore(self.now.clone())
        }
    }
}
