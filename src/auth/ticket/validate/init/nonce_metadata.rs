use actix_web::HttpRequest;
use tonic::metadata::MetadataMap;

use crate::auth::ticket::kernel::{
    x_actix_web::header::HEADER_NONCE, x_tonic::metadata::METADATA_NONCE,
};

use crate::z_lib::request::{helper::header, x_tonic::metadata::metadata};

use crate::auth::ticket::validate::infra::AuthNonceMetadata;

use crate::{auth::ticket::kernel::data::AuthNonce, z_lib::request::data::MetadataError};

pub struct ActixWebAuthNonceMetadata<'a> {
    request: &'a HttpRequest,
}

impl<'a> ActixWebAuthNonceMetadata<'a> {
    pub const fn new(request: &'a HttpRequest) -> Self {
        Self { request }
    }
}

impl<'a> AuthNonceMetadata for ActixWebAuthNonceMetadata<'a> {
    fn nonce(&self) -> Result<Option<AuthNonce>, MetadataError> {
        header(self.request, HEADER_NONCE).map(|value| value.map(AuthNonce::restore))
    }
}

pub struct TonicAuthNonceMetadata<'a> {
    metadata: &'a MetadataMap,
}

impl<'a> TonicAuthNonceMetadata<'a> {
    pub const fn new(metadata: &'a MetadataMap) -> Self {
        Self { metadata }
    }
}

impl<'a> AuthNonceMetadata for TonicAuthNonceMetadata<'a> {
    fn nonce(&self) -> Result<Option<AuthNonce>, MetadataError> {
        metadata(&self.metadata, METADATA_NONCE)
            .map(|value| value.map(|nonce| AuthNonce::restore(nonce.into())))
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::ticket::validate::infra::AuthNonceMetadata;

    use crate::{auth::ticket::kernel::data::AuthNonce, z_lib::request::data::MetadataError};

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
