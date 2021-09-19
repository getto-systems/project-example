#[cfg(test)]
pub mod test {
    use crate::auth::auth_ticket::_common::kernel::infra::AuthTokenMetadata;

    use crate::{
        auth::auth_ticket::_common::kernel::data::AuthToken,
        z_details::_common::request::data::MetadataError,
    };

    pub struct StaticAuthTokenMetadata {
        token: AuthToken,
    }
    impl StaticAuthTokenMetadata {
        pub const fn new(token: String) -> Self {
            Self {
                token: AuthToken::restore(token),
            }
        }
    }

    impl AuthTokenMetadata for StaticAuthTokenMetadata {
        fn token(&self) -> Result<Option<AuthToken>, MetadataError> {
            Ok(Some(self.token.clone()))
        }
    }
}
