mod nonce;

use crate::auth::x_outside_feature::feature::AuthOutsideStore;

use crate::auth::ticket::validate::init::nonce_repository::dynamodb::nonce::TableNonce;

use crate::auth::ticket::validate::infra::AuthNonceRepository;

use crate::{
    auth::ticket::kernel::data::{AuthDateTime, AuthNonce, ExpireDateTime},
    z_lib::repository::data::{RegisterResult, RepositoryError},
};

pub struct DynamoDbAuthNonceRepository<'a> {
    nonce: TableNonce<'a>,
}

impl<'a> DynamoDbAuthNonceRepository<'a> {
    pub const fn new(feature: &'a AuthOutsideStore) -> Self {
        Self {
            nonce: TableNonce::new(feature),
        }
    }
}

#[async_trait::async_trait]
impl<'a> AuthNonceRepository for DynamoDbAuthNonceRepository<'a> {
    async fn register(
        &self,
        nonce: AuthNonce,
        expires: ExpireDateTime,
        registered_at: AuthDateTime,
    ) -> Result<RegisterResult<()>, RepositoryError> {
        self.nonce.put_nonce(nonce, expires, registered_at).await
    }
}
