use crate::auth::kernel::data::AuthDateTime;

pub trait AuthClock {
    fn now(&self) -> AuthDateTime;
}
