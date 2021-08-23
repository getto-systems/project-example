use std::collections::HashMap;

use crate::auth::auth_ticket::_common::y_protobuf::service::{
    AuthTokenEncodedPb, AuthTokenPb, CloudfrontTokenKindPb, CloudfrontTokenPb,
};

use crate::auth::auth_ticket::_common::kernel::data::{
    AuthTokenEncoded, AuthTokenExtract, CloudfrontTokenKind,
};

impl Into<AuthTokenEncodedPb> for AuthTokenEncoded {
    fn into(self) -> AuthTokenEncodedPb {
        AuthTokenEncodedPb {
            ticket_token: Some(self.ticket_token.into()),
            api_token: Some(self.api_token.into()),

            cloudfront_tokens: self
                .cloudfront_tokens
                .into_iter()
                .map(|(kind, token)| {
                    let kind: CloudfrontTokenKindPb = kind.into();
                    CloudfrontTokenPb {
                        kind: kind as i32,
                        token: Some(token.into()),
                    }
                })
                .collect(),
        }
    }
}

impl Into<Option<AuthTokenEncoded>> for AuthTokenEncodedPb {
    fn into(self) -> Option<AuthTokenEncoded> {
        match (self.ticket_token, self.api_token) {
            (Some(ticket_token), Some(api_token)) => {
                let mut cloudfront_tokens = HashMap::new();
                self.cloudfront_tokens
                    .into_iter()
                    .for_each(|cloudfront_token| {
                        if let Some((kind, token)) = cloudfront_token.into() {
                            cloudfront_tokens.insert(kind, token);
                        }
                    });

                Some(AuthTokenEncoded {
                    ticket_token: ticket_token.into(),
                    api_token: api_token.into(),
                    cloudfront_tokens,
                })
            }
            _ => None,
        }
    }
}

impl Into<AuthTokenPb> for AuthTokenExtract {
    fn into(self) -> AuthTokenPb {
        AuthTokenPb {
            token: self.token,
            expires: self.expires,
        }
    }
}

impl Into<AuthTokenExtract> for AuthTokenPb {
    fn into(self) -> AuthTokenExtract {
        AuthTokenExtract {
            token: self.token,
            expires: self.expires,
        }
    }
}

impl Into<Option<(CloudfrontTokenKind, AuthTokenExtract)>> for CloudfrontTokenPb {
    fn into(self) -> Option<(CloudfrontTokenKind, AuthTokenExtract)> {
        match (CloudfrontTokenKindPb::from_i32(self.kind), self.token) {
            (Some(kind), Some(token)) => Some((kind.into(), token.into())),
            _ => None,
        }
    }
}

impl Into<CloudfrontTokenKindPb> for CloudfrontTokenKind {
    fn into(self) -> CloudfrontTokenKindPb {
        match self {
            Self::KeyPairId => CloudfrontTokenKindPb::KeyPairId,
            Self::Policy => CloudfrontTokenKindPb::Policy,
            Self::Signature => CloudfrontTokenKindPb::Signature,
        }
    }
}

impl Into<CloudfrontTokenKind> for CloudfrontTokenKindPb {
    fn into(self) -> CloudfrontTokenKind {
        match self {
            Self::KeyPairId => CloudfrontTokenKind::KeyPairId,
            Self::Policy => CloudfrontTokenKind::Policy,
            Self::Signature => CloudfrontTokenKind::Signature,
        }
    }
}