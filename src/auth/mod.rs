pub mod proxy;
mod ticket;
mod user;
pub mod x_actix_web;
pub mod x_outside_feature;
pub mod x_tonic;

pub mod data {
    pub use crate::auth::{
        ticket::kernel::data::{ExpansionLimitDuration, ExpireDuration},
        user::kernel::data::RequireAuthRoles,
    };
}
pub mod infra {
    pub use crate::auth::ticket::validate::infra::AuthMetadataContent;
}
pub mod init {
    pub use crate::auth::ticket::validate::init::{
        CheckPermissionStruct, ValidateApiMetadataStruct,
    };

    #[cfg(test)]
    pub mod test {
        pub use crate::auth::ticket::validate::init::{
            auth_metadata::test::StaticAuthMetadata, test::StaticCheckPermissionStruct,
            token_decoder::test::StaticAuthTokenDecoder,
            validate_service::test::StaticValidateService,
        };
    }
}
pub mod method {
    pub use crate::auth::ticket::validate::method::{
        authorize, validate_auth_metadata, AuthorizeEvent, AuthorizeInfra,
        ValidateAuthMetadataEvent, ValidateAuthMetadataInfra,
    };
}
