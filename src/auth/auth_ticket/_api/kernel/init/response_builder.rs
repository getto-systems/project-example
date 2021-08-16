use crate::auth::_api::x_outside_feature::feature::AuthOutsideCookie;

use crate::auth::auth_ticket::_api::kernel::infra::AuthTokenResponseBuilder;

use crate::auth::auth_ticket::_api::kernel::data::{AuthTokenResponse, AuthTokenMessage};

pub struct CookieAuthTokenResponseBuilder<'a> {
    domain: &'a str,
}

impl<'a> CookieAuthTokenResponseBuilder<'a> {
    pub const fn new(feature: &'a AuthOutsideCookie) -> Self {
        Self {
            domain: feature.domain,
        }
    }
}

impl<'a> AuthTokenResponseBuilder for CookieAuthTokenResponseBuilder<'a> {
    fn build(&self, message: AuthTokenMessage) -> AuthTokenResponse {
        AuthTokenResponse {
            domain: self.domain.into(),
            message,
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::auth_ticket::_api::kernel::infra::AuthTokenResponseBuilder;

    use crate::auth::auth_ticket::_api::kernel::data::{AuthTokenResponse, AuthTokenMessage};

    pub struct StaticAuthTokenResponseBuilder;

    impl AuthTokenResponseBuilder for StaticAuthTokenResponseBuilder {
        fn build(&self, message: AuthTokenMessage) -> AuthTokenResponse {
            AuthTokenResponse {
                domain: "DOMAIN".into(),
                message,
            }
        }
    }
}
