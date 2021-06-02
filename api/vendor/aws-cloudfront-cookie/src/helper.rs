use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use pem::{parse, PemError};
use ring::{error::KeyRejected, signature::{RSA_PKCS1_SHA512, RsaKeyPair}};

use crate::{
    data::{Condition, ConditionDateLessThan, Policy, Statement},
    sign::Key,
};

impl Key {
    pub fn pkcs1_sha512_from_pem(private_key_pem: impl AsRef<[u8]>) -> Result<Self, KeyParseError> {
        let pem = parse(private_key_pem).map_err(KeyParseError::PemError)?;

        let key_pair =
            RsaKeyPair::from_der(pem.contents.as_ref()).map_err(KeyParseError::KeyRejected)?;

        Ok(Self::new(key_pair, &RSA_PKCS1_SHA512))
    }
}

#[derive(Debug)]
pub enum KeyParseError {
    PemError(PemError),
    KeyRejected(KeyRejected),
}

impl Display for KeyParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::PemError(err) => write!(f, "{}", err),
            Self::KeyRejected(err) => write!(f, "{}", err),
        }
    }
}
impl Error for KeyParseError {}

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
