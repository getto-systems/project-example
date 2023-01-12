use rsa::{
    pkcs1::{DecodeRsaPrivateKey, Error as Pkcs1Error},
    RsaPrivateKey,
};

use crate::{
    data::{Condition, ConditionDateLessThan, Policy, Statement},
    sign::Key,
};

impl Key {
    pub fn from_pem(private_key_pem: &str) -> Result<Self, KeyParseError> {
        let private_key =
            RsaPrivateKey::from_pkcs1_pem(private_key_pem).map_err(KeyParseError::KeyError)?;

        Ok(Self::new(private_key))
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

impl Policy {
    pub fn from_resource(resource: String, expires: i64) -> Self {
        Self {
            statement: vec![Statement {
                resource,
                condition: Condition {
                    date_less_than: ConditionDateLessThan {
                        epoch_time: expires,
                    },
                },
            }],
        }
    }
}
