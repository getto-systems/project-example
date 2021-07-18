use tonic::Request;

use crate::auth::auth_ticket::_common::x_tonic::metadata::METADATA_NONCE;

use crate::z_details::_auth::request::helper::metadata;

use crate::auth::auth_ticket::_auth::kernel::infra::AuthNonceMetadata;

use crate::auth::auth_ticket::_auth::kernel::data::AuthNonceValue;
use crate::z_details::_auth::request::data::MetadataError;

pub struct TonicAuthNonceMetadata<'a, T> {
    request: &'a Request<T>,
}

impl<'a, T> TonicAuthNonceMetadata<'a, T> {
    pub const fn new(request: &'a Request<T>) -> Self {
        Self { request }
    }
}

impl<'a, T> AuthNonceMetadata for TonicAuthNonceMetadata<'a, T> {
    fn nonce(&self) -> Result<AuthNonceValue, MetadataError> {
        metadata(self.request, METADATA_NONCE).map(AuthNonceValue::new)
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::auth_ticket::_auth::kernel::infra::AuthNonceMetadata;

    use crate::auth::auth_ticket::_auth::kernel::data::AuthNonceValue;
    use crate::z_details::_auth::request::data::MetadataError;

    pub enum StaticAuthNonceMetadata {
        Valid(AuthNonceValue),
        NotFound, // TODO このテストを追加する必要がある
    }

    impl AuthNonceMetadata for StaticAuthNonceMetadata {
        fn nonce(&self) -> Result<AuthNonceValue, MetadataError> {
            match self {
                Self::NotFound => Err(MetadataError::NotFound),
                Self::Valid(nonce) => Ok(nonce.clone()),
            }
        }
    }
}
