mod kernel;
mod proxy;
mod ticket;
mod user;
pub mod x_actix_web;
pub mod x_outside_feature;
pub mod x_tonic;

pub mod data {
    pub use crate::auth::{
        kernel::data::{ExpansionLimitDuration, ExpireDuration},
        ticket::kernel::data::{AuthPermissionRequired, AuthorizeTokenExtract},
    };
}
pub mod method {
    pub use crate::auth::ticket::authorize::method::{
        authorize_with_token, AuthorizeWithTokenEvent, AuthorizeWithTokenInfra,
    };

    pub mod proxy {
        pub use crate::auth::ticket::authorize::proxy::{
            authorize, AuthorizeEvent, AuthorizeInfra,
        };
    }
}
pub mod init {
    pub use crate::auth::ticket::authorize::init::{
        ActiveAuthorizeInfra, ActiveAuthorizeWithTokenInfra,
    };

    #[cfg(test)]
    pub mod test {
        pub use crate::auth::ticket::{
            authorize::init::test::{StaticAuthorizeInfra, StaticAuthorizeWithTokenInfra},
            kernel::init::request::test::StaticAuthorizeToken,
        };
    }
}
