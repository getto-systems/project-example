use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use digest::{Digest, Update};
use rand::thread_rng;
use rsa::{errors::Error as RsaError, traits::SignatureScheme, Pkcs1v15Sign};
use serde_json::{to_string, Error as SerdeJsonError};
use sha1::Sha1;

use crate::auth::ticket::kernel::aws::cloudfront::data::{
    AwsCloudfrontKey, AwsCloudfrontPolicy, AwsCloudfrontSignedContent,
};

impl AwsCloudfrontKey {
    pub fn sign(
        &self,
        policy: AwsCloudfrontPolicy,
    ) -> Result<AwsCloudfrontSignedContent, AwsCloudfrontKeyError> {
        let policy = to_string(&policy)?;

        let mut rng = thread_rng();
        let signature = Pkcs1v15Sign::new::<Sha1>().sign(
            Some(&mut rng),
            &self.private_key,
            Sha1::new().chain(policy.as_bytes()).finalize().as_ref(),
        )?;

        Ok(AwsCloudfrontSignedContent {
            policy: cloudfront_base64(policy),
            signature: cloudfront_base64(&signature),
        })
    }
}

fn cloudfront_base64(source: impl AsRef<[u8]>) -> String {
    URL_SAFE_NO_PAD.encode(source)
}

pub enum AwsCloudfrontKeyError {
    SerializeError(SerdeJsonError),
    SignError(RsaError),
}

impl std::fmt::Display for AwsCloudfrontKeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::SerializeError(err) => write!(f, "{}", err),
            Self::SignError(err) => write!(f, "{}", err),
        }
    }
}

impl From<SerdeJsonError> for AwsCloudfrontKeyError {
    fn from(value: SerdeJsonError) -> Self {
        Self::SerializeError(value)
    }
}

impl From<RsaError> for AwsCloudfrontKeyError {
    fn from(value: RsaError) -> Self {
        Self::SignError(value)
    }
}
