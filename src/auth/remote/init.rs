pub use crate::auth::ticket::validate::init::{
    ValidateApiMetadataStruct, CheckPermissionStruct,
};

#[cfg(test)]
pub mod test {
    pub use crate::auth::ticket::validate::init::{
        auth_metadata::test::StaticAuthMetadata, test::StaticValidateApiTokenStruct,
        token_decoder::test::StaticAuthTokenDecoder, validate_service::test::StaticValidateService,
    };
}
