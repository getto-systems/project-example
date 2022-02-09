use crate::auth::x_outside_feature::proxy::feature::AuthOutsideCookie;

use crate::auth::ticket::kernel::infra::AuthTokenResponseBuilder;

use crate::auth::ticket::kernel::data::{AuthTokenMessage, AuthTokenResponse};

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
    use crate::auth::ticket::kernel::infra::AuthTokenResponseBuilder;

    use crate::auth::ticket::kernel::data::{AuthTokenMessage, AuthTokenResponse};

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
