#[cfg(test)]
pub mod test {
    use crate::auth::auth_ticket::_common::kernel::infra::AuthNonceMetadata;

    use crate::{
        auth::auth_ticket::_common::kernel::data::AuthNonce,
        z_details::_common::request::data::MetadataError,
    };

    pub struct StaticAuthNonceMetadata {
        nonce: AuthNonce,
    }
    impl StaticAuthNonceMetadata {
        pub const fn new(nonce: String) -> Self {
            Self {
                nonce: AuthNonce::restore(nonce),
            }
        }
    }

    impl AuthNonceMetadata for StaticAuthNonceMetadata {
        fn nonce(&self) -> Result<Option<AuthNonce>, MetadataError> {
            Ok(Some(self.nonce.clone()))
        }
    }
}
