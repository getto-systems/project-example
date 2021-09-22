pub use crate::auth::auth_ticket::_common::validate::init::ValidateApiTokenStruct;

#[cfg(test)]
pub mod test {
    pub use crate::auth::auth_ticket::_common::{
        kernel::init::{
            auth_metadata::test::StaticAuthMetadata, token_decoder::test::StaticAuthTokenDecoder,
        },
        validate::init::{
            test::StaticValidateApiTokenStruct, validate_service::test::StaticValidateService,
        },
    };
}
