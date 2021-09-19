#[cfg(test)]
pub mod test {
    use crate::auth::auth_ticket::_common::kernel::infra::{
        AuthServiceMetadata, AuthServiceMetadataContent,
    };

    use crate::auth::auth_ticket::_common::kernel::data::{
        AuthNonce, AuthServiceMetadataError, AuthToken,
    };

    pub struct StaticAuthServiceMetadata {
        pub nonce: String,
        pub token: String,
    }

    impl AuthServiceMetadata for StaticAuthServiceMetadata {
        fn metadata(&self) -> Result<AuthServiceMetadataContent, AuthServiceMetadataError> {
            Ok(AuthServiceMetadataContent {
                nonce: Some(AuthNonce::restore(self.nonce.clone())),
                token: Some(AuthToken::restore(self.token.clone())),
            })
        }
    }
}
