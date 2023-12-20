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
        ticket::{
            authorize::{
                data::{
                    AuthorizeSuccess, AuthorizeWithTokenError, CheckAuthorizeTokenError,
                    CheckAuthorizeTokenSuccess,
                },
                proxy::data::AuthorizeProxyError,
            },
            kernel::{
                aws::cloudfront::data::AwsCloudfrontKey,
                data::{
                    AuthPermissionRequired, AuthorizeTokenExtract, ValidateAuthorizeTokenError,
                },
            },
        },
    };
}
pub mod action {
    pub use crate::auth::ticket::authorize::{
        action::CheckAuthorizeTokenAction, proxy::action::AuthorizeProxyAction,
    };
}
pub mod feature {
    pub use crate::auth::kernel::feature::{AsAuthorizedInfra, AsCheckedInfra};
}
