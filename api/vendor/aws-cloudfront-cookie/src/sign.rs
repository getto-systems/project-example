use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use base64::{encode_config, STANDARD};
use ring::{
    error::Unspecified,
    rand::SystemRandom,
    signature::{RsaEncoding, RsaKeyPair},
};
use serde_json::{to_string, Error as SerdeJsonError};

use crate::data::{Policy, SignedContent};

pub struct Key {
    key_pair: RsaKeyPair,
    padding_algorithm: &'static dyn RsaEncoding,
}

impl Key {
    pub fn new(key_pair: RsaKeyPair, padding_algorithm: &'static dyn RsaEncoding) -> Self {
        Self {
            key_pair,
            padding_algorithm,
        }
    }
}

impl Key {
    pub fn sign(&self, policy: Policy) -> Result<SignedContent, SignError> {
        let policy = to_string(&policy).map_err(SignError::SerializeError)?;

        let random_generator = SystemRandom::new();

        let mut signature = vec![0; self.key_pair.public_modulus_len()];
        self.key_pair
            .sign(
                self.padding_algorithm,
                &random_generator,
                policy.as_bytes(),
                &mut signature,
            )
            .map_err(SignError::SignError)?;

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
pub enum SignError {
    SerializeError(SerdeJsonError),
    SignError(Unspecified),
}

impl Display for SignError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::SerializeError(err) => write!(f, "{}", err),
            Self::SignError(err) => write!(f, "{}", err),
        }
    }
}
impl Error for SignError {}
