use actix_web::HttpRequest;
use tonic::metadata::MetadataMap;

use crate::z_lib::remote::request::x_tonic::metadata::metadata;

use crate::auth::ticket::remote::kernel::x_tonic::metadata::{METADATA_NONCE, METADATA_TOKEN};

use crate::auth::ticket::remote::kernel::init::{
    nonce_metadata::ActixWebAuthNonceMetadata,
    token_metadata::{ApiTokenMetadata, TicketTokenMetadata},
};

use crate::auth::ticket::remote::kernel::infra::{
    AuthMetadata, AuthMetadataContent, AuthNonceMetadata, AuthTokenMetadata,
};

use crate::{
    auth::ticket::remote::kernel::data::{AuthNonce, AuthToken},
    z_lib::remote::request::data::MetadataError,
};

pub struct TicketAuthMetadata<'a> {
    nonce_metadata: ActixWebAuthNonceMetadata<'a>,
    token_metadata: TicketTokenMetadata<'a>,
}

impl<'a> TicketAuthMetadata<'a> {
    pub const fn new(request: &'a HttpRequest) -> Self {
        Self {
            nonce_metadata: ActixWebAuthNonceMetadata::new(request),
            token_metadata: TicketTokenMetadata::new(request),
        }
    }
}

impl<'a> AuthMetadata for TicketAuthMetadata<'a> {
    fn metadata(&self) -> Result<AuthMetadataContent, MetadataError> {
        Ok(AuthMetadataContent {
            nonce: self.nonce_metadata.nonce()?,
            token: self.token_metadata.token()?,
        })
    }
}

pub struct ApiAuthMetadata<'a> {
    nonce_metadata: ActixWebAuthNonceMetadata<'a>,
    token_metadata: ApiTokenMetadata<'a>,
}

impl<'a> ApiAuthMetadata<'a> {
    pub const fn new(request: &'a HttpRequest) -> Self {
        Self {
            nonce_metadata: ActixWebAuthNonceMetadata::new(request),
            token_metadata: ApiTokenMetadata::new(request),
        }
    }
}

impl<'a> AuthMetadata for ApiAuthMetadata<'a> {
    fn metadata(&self) -> Result<AuthMetadataContent, MetadataError> {
        Ok(AuthMetadataContent {
            nonce: self.nonce_metadata.nonce()?,
            token: self.token_metadata.token()?,
        })
    }
}

pub struct NoAuthMetadata<'a> {
    nonce_metadata: ActixWebAuthNonceMetadata<'a>,
}

impl<'a> NoAuthMetadata<'a> {
    pub const fn new(request: &'a HttpRequest) -> Self {
        Self {
            nonce_metadata: ActixWebAuthNonceMetadata::new(request),
        }
    }
}

impl<'a> AuthMetadata for NoAuthMetadata<'a> {
    fn metadata(&self) -> Result<AuthMetadataContent, MetadataError> {
        Ok(AuthMetadataContent {
            nonce: self.nonce_metadata.nonce()?,
            token: None,
        })
    }
}

pub struct TonicAuthMetadata<'a> {
    metadata: &'a MetadataMap,
}

impl<'a> TonicAuthMetadata<'a> {
    pub const fn new(metadata: &'a MetadataMap) -> Self {
        Self { metadata }
    }
}

impl<'a> AuthMetadata for TonicAuthMetadata<'a> {
    fn metadata(&self) -> Result<AuthMetadataContent, MetadataError> {
        Ok(AuthMetadataContent {
            nonce: fetch_metadata(&self.metadata, METADATA_NONCE, AuthNonce::restore)?,
            token: fetch_metadata(&self.metadata, METADATA_TOKEN, AuthToken::restore)?,
        })
    }
}

fn fetch_metadata<T>(
    map: &MetadataMap,
    key: &str,
    converter: impl Fn(String) -> T,
) -> Result<Option<T>, MetadataError> {
    metadata(map, key).map(|value| value.map(|value| converter(value.into())))
}

#[cfg(test)]
pub mod test {
    use crate::auth::ticket::remote::kernel::infra::{AuthMetadata, AuthMetadataContent};

    use crate::{
        auth::ticket::remote::kernel::data::{AuthNonce, AuthToken},
        z_lib::remote::request::data::MetadataError,
    };

    pub struct StaticAuthMetadata {
        pub nonce: String,
        pub token: String,
    }

    impl AuthMetadata for StaticAuthMetadata {
        fn metadata(&self) -> Result<AuthMetadataContent, MetadataError> {
            Ok(AuthMetadataContent {
                nonce: Some(AuthNonce::restore(self.nonce.clone())),
                token: Some(AuthToken::restore(self.token.clone())),
            })
        }
    }
}
