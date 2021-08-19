pub use crate::auth::auth_ticket::_api::validate::init::ValidateApiTokenStruct;

#[cfg(test)]
pub mod test {
    pub use crate::auth::auth_ticket::_api::{
        kernel::init::{
            nonce_header::test::StaticAuthNonceHeader, token_header::test::StaticAuthTokenHeader,
        },
        validate::init::{
            test::StaticValidateApiTokenStruct, validate_service::test::StaticValidateService,
        },
    };
}
