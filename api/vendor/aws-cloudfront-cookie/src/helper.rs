use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use pem::{parse, PemError};
use ring::{
    error::KeyRejected,
    signature::{RsaKeyPair, RSA_PKCS1_SHA512},
};
use rsa::RSAPrivateKey;

use crate::{
    data::{Condition, ConditionDateLessThan, Policy, Statement},
    sign::Key,
};

impl Key {
    pub fn pkcs1_sha512_from_pem(private_key_pem: impl AsRef<[u8]>) -> Result<Self, KeyParseError> {
        let pem = parse(private_key_pem).map_err(KeyParseError::PemError)?;

        let private_key =
            RSAPrivateKey::from_pkcs1(pem.contents.as_ref()).map_err(KeyParseError::KeyError)?;

        // let key_pair =
        //     RsaKeyPair::from_der(pem.contents.as_ref()).map_err(KeyParseError::KeyRejected)?;

        Ok(Self::new(private_key))
    }
}

#[derive(Debug)]
pub enum KeyParseError {
    PemError(PemError),
    KeyError(rsa::errors::Error)
    // KeyRejected(KeyRejected),
}

impl Display for KeyParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::PemError(err) => write!(f, "{}", err),
            Self::KeyError(err) => write!(f, "{}", err),
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
