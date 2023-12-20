use rsa::{
    pkcs1::{DecodeRsaPrivateKey, Error as Pkcs1Error},
    RsaPrivateKey,
};

use crate::auth::ticket::kernel::aws::cloudfront::data::{
    AwsCloudfrontCondition, AwsCloudfrontConditionDateLessThan, AwsCloudfrontKey,
    AwsCloudfrontPolicy, AwsCloudfrontStatement,
};

impl AwsCloudfrontKey {
    pub fn from_pem(private_key_pem: &str) -> Result<Self, KeyParseError> {
        let private_key =
            RsaPrivateKey::from_pkcs1_pem(private_key_pem).map_err(KeyParseError::KeyError)?;

        Ok(Self { private_key })
    }
}

#[derive(Debug)]
pub enum KeyParseError {
    KeyError(Pkcs1Error),
}

impl std::fmt::Display for KeyParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::KeyError(err) => err.fmt(f),
        }
    }
}

impl AwsCloudfrontPolicy {
    pub fn from_resource(resource: String, expires: i64) -> Self {
        Self {
            statement: vec![AwsCloudfrontStatement {
                resource,
                condition: AwsCloudfrontCondition {
                    date_less_than: AwsCloudfrontConditionDateLessThan {
                        epoch_time: expires,
                    },
                },
            }],
        }
    }
}
