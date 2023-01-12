use base64::encode;
use digest::{Digest, Update};
use rsa::{errors::Error as RsaError, Hash, PaddingScheme, RsaPrivateKey};
use serde_json::{to_string, Error as SerdeJsonError};
use sha1::Sha1;

use crate::data::{Policy, SignedContent};

pub struct Key {
    private_key: RsaPrivateKey,
}

impl Key {
    pub fn new(private_key: RsaPrivateKey) -> Self {
        Self { private_key }
    }

    pub fn sign(&self, policy: Policy) -> Result<SignedContent, KeyError> {
        let policy = to_string(&policy).map_err(KeyError::SerializeError)?;

        let (padding, hash) = hash_sha1(policy.as_bytes());
        let signature = self
            .private_key
            .sign(padding, hash.as_ref())
            .map_err(|err| KeyError::SignError(err))?;

        Ok(SignedContent {
            policy: cloudfront_base64(policy),
            signature: cloudfront_base64(signature),
        })
    }
}

fn hash_sha1(message: &[u8]) -> (PaddingScheme, impl AsRef<[u8]>) {
    (
        PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA1)),
        Sha1::new().chain(message).finalize(),
    )
}

fn cloudfront_base64(source: impl AsRef<[u8]>) -> String {
    // cloudfront flavored base64; see: https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/private-content-setting-signed-cookie-custom-policy.html#private-content-custom-policy-signature-cookies
    encode(source)
        .replace("+", "-")
        .replace("=", "_")
        .replace("/", "~")
}

pub enum KeyError {
    SerializeError(SerdeJsonError),
    SignError(RsaError),
}

impl std::fmt::Display for KeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::SerializeError(err) => write!(f, "{}", err),
            Self::SignError(err) => write!(f, "{}", err),
        }
    }
}
