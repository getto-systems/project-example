pub use crate::auth::ticket::remote::validate::init::{
    ValidateApiMetadataStruct, ValidateApiTokenStruct,
};

#[cfg(test)]
pub mod test {
    pub use crate::auth::ticket::remote::validate::init::{
        auth_metadata::test::StaticAuthMetadata, test::StaticValidateApiTokenStruct,
        token_decoder::test::StaticAuthTokenDecoder, validate_service::test::StaticValidateService,
    };
}
