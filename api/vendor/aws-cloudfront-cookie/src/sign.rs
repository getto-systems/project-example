use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use base64::{encode_config, STANDARD};
use digest::Digest;
use rsa::{Hash, PaddingScheme, RSAPrivateKey, errors::Error as RsaError};
use serde_json::{to_string, Error as SerdeJsonError};
use sha1::Sha1;

use crate::data::{Policy, SignedContent};

pub struct Key {
    private_key: RSAPrivateKey,
}

impl Key {
    pub fn new(private_key: RSAPrivateKey) -> Self {
        Self { private_key }
    }
}

impl Key {
    pub fn sign_sha1(&self, policy: Policy) -> Result<SignedContent, KeyError> {
        let policy = to_string(&policy).map_err(KeyError::SerializeError)?;

        let padding = PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA1));

        let hash = Sha1::new().chain(policy.as_bytes()).finalize();

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

fn cloudfront_base64(source: impl AsRef<[u8]>) -> String {
    // cloudfront flavored base64; see: https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/private-content-setting-signed-cookie-custom-policy.html#private-content-custom-policy-signature-cookies
    encode_config(source, STANDARD)
        .replace("+", "-")
        .replace("=", "_")
        .replace("/", "~")
}

#[derive(Debug)]
pub enum KeyError {
    SerializeError(SerdeJsonError),
    SignError(RsaError),
}

impl Display for KeyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::SerializeError(err) => write!(f, "{}", err),
            Self::SignError(err) => write!(f, "{}", err),
        }
    }
}
impl Error for KeyError {}
