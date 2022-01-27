pub use crate::auth::ticket::remote::{
    validate::init::ValidateApiTokenStruct, validate_metadata::init::ValidateApiMetadataStruct,
};

#[cfg(test)]
pub mod test {
    pub use crate::auth::ticket::remote::{
        kernel::init::{
            auth_metadata::test::StaticAuthMetadata, token_decoder::test::StaticAuthTokenDecoder,
        },
        validate::init::{
            test::StaticValidateApiTokenStruct, validate_service::test::StaticValidateService,
        },
    };
}
