use crate::auth::_api::x_outside_feature::feature::AuthOutsideCookie;

use crate::auth::auth_ticket::_api::kernel::infra::AuthTokenMessenger;

use crate::auth::auth_ticket::_api::kernel::data::{AuthTokenMessage, AuthTokenMessageEncoded};

pub struct CookieAuthTokenMessenger<'a> {
    domain: &'a str,
}

impl<'a> CookieAuthTokenMessenger<'a> {
    pub const fn new(feature: &'a AuthOutsideCookie) -> Self {
        Self {
            domain: feature.domain,
        }
    }
}

impl<'a> AuthTokenMessenger for CookieAuthTokenMessenger<'a> {
    fn to_message(&self, message: AuthTokenMessageEncoded) -> AuthTokenMessage {
        AuthTokenMessage {
            domain: self.domain.into(),
            message,
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::auth_ticket::_api::kernel::infra::AuthTokenMessenger;

    use crate::auth::auth_ticket::_api::kernel::data::{AuthTokenMessage, AuthTokenMessageEncoded};

    pub struct StaticAuthTokenMessenger;

    impl AuthTokenMessenger for StaticAuthTokenMessenger {
        fn to_message(&self, message: AuthTokenMessageEncoded) -> AuthTokenMessage {
            AuthTokenMessage {
                domain: "DOMAIN".into(),
                message,
            }
        }
    }
}
