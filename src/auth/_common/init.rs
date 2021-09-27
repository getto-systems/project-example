pub use crate::auth::auth_ticket::remote::validate_api_token::init::ValidateApiTokenStruct;

#[cfg(test)]
pub mod test {
    pub use crate::auth::auth_ticket::remote::{
        kernel::init::{
            auth_metadata::test::StaticAuthMetadata, token_decoder::test::StaticAuthTokenDecoder,
        },
        validate_api_token::init::{
            test::StaticValidateApiTokenStruct, validate_service::test::StaticValidateService,
        },
    };
}
