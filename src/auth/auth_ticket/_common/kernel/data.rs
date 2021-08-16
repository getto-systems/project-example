use std::collections::HashMap;

#[derive(Clone)]
pub struct AuthNonce(String);

impl AuthNonce {
    pub const fn new(nonce: String) -> Self {
        Self(nonce)
    }

    pub fn extract(self) -> String {
        self.0
    }

    #[cfg(test)]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Clone)]
pub struct AuthToken(String);

impl AuthToken {
    pub const fn new(token: String) -> Self {
        Self(token)
    }

    pub fn extract(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Clone)]
pub struct AuthTokenExtract {
    pub token: String,
    pub expires: i64,
}

pub struct AuthTokenEncoded {
    pub ticket_token: AuthTokenExtract,
    pub api_token: AuthTokenExtract,
    pub cloudfront_tokens: HashMap<CloudfrontTokenKind, AuthTokenExtract>,
}

#[derive(Eq, PartialEq, Hash)]
pub enum CloudfrontTokenKind {
    KeyPairId,
    Policy,
    Signature,
}
