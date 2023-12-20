use tonic::{Response, Status};

use crate::auth::ticket::y_protobuf::service::{
    AuthPermissionGrantedPb, AuthTokenPb, AuthenticateTokenPb, AuthorizeTokenPb, CdnTokenPb,
};

use crate::common::api::response::x_tonic::ServiceResponder;

use crate::x_content::permission::AuthPermission;

use crate::auth::{
    kernel::data::ExpireDateTime,
    ticket::kernel::data::{
        AWSCloudfrontToken, AuthPermissionError, AuthPermissionGranted, AuthToken,
        AuthenticateToken, AuthorizeToken, CdnToken, DecodeAuthenticateTokenError,
        DecodeAuthorizeTokenError, ValidateAuthPermissionError, ValidateAuthenticateTokenError,
        ValidateAuthorizeTokenError,
    },
};

impl<R> ServiceResponder<R> for ValidateAuthenticateTokenError {
    fn respond_to(self) -> Result<Response<R>, Status> {
        match self {
            Self::NotFound => Err(Status::unauthenticated("authenticate token not found")),
            Self::MetadataError(_) => {
                Err(Status::unauthenticated("authenticate token metadata error"))
            }
        }
    }
}

impl<R> ServiceResponder<R> for DecodeAuthenticateTokenError {
    fn respond_to(self) -> Result<Response<R>, Status> {
        match self {
            Self::Expired => Err(Status::unauthenticated("authenticate token has expired")),
            Self::Invalid(_) => Err(Status::unauthenticated("invalid authenticate token")),
        }
    }
}

impl<R> ServiceResponder<R> for ValidateAuthorizeTokenError {
    fn respond_to(self) -> Result<Response<R>, Status> {
        match self {
            Self::NotFound => Err(Status::unauthenticated("authorize token not found")),
            Self::MetadataError(_) => {
                Err(Status::unauthenticated("authorize token metadata error"))
            }
        }
    }
}

impl<R> ServiceResponder<R> for DecodeAuthorizeTokenError {
    fn respond_to(self) -> Result<Response<R>, Status> {
        match self {
            Self::Expired => Err(Status::unauthenticated("authorize token has expired")),
            Self::Invalid(_) => Err(Status::unauthenticated("invalid authorize token")),
        }
    }
}

impl<R> ServiceResponder<R> for ValidateAuthPermissionError {
    fn respond_to(self) -> Result<Response<R>, Status> {
        match self {
            Self::Invalid => Err(Status::unauthenticated("invalid auth permission")),
        }
    }
}

impl<R> ServiceResponder<R> for AuthPermissionError {
    fn respond_to(self) -> Result<Response<R>, Status> {
        match self {
            Self::PermissionDenied(_, _) => Err(Status::unauthenticated("permission denied")),
        }
    }
}

impl Into<AuthPermissionGrantedPb> for AuthPermissionGranted {
    fn into(self) -> AuthPermissionGrantedPb {
        AuthPermissionGrantedPb {
            permissions: self.extract().into_iter().collect(),
        }
    }
}

impl Into<AuthPermissionGranted> for AuthPermissionGrantedPb {
    fn into(self) -> AuthPermissionGranted {
        AuthPermissionGranted::restore(
            self.permissions
                .into_iter()
                .filter_map(AuthPermission::convert)
                .collect(),
        )
    }
}

impl Into<AuthTokenPb> for AuthToken {
    fn into(self) -> AuthTokenPb {
        AuthTokenPb {
            authenticate_token: Some(self.authenticate_token.into()),
            authorize_token: Some(self.authorize_token.into()),
            cdn_token: Some(self.cdn_token.into()),
        }
    }
}

impl From<(AuthenticateToken, ExpireDateTime)> for AuthenticateTokenPb {
    fn from(value: (AuthenticateToken, ExpireDateTime)) -> Self {
        AuthenticateTokenPb {
            expires: value.1.extract_to_timestamp(),
            token: value.0.extract(),
        }
    }
}

impl From<(AuthorizeToken, ExpireDateTime)> for AuthorizeTokenPb {
    fn from(value: (AuthorizeToken, ExpireDateTime)) -> Self {
        AuthorizeTokenPb {
            expires: value.1.extract_to_timestamp(),
            token: value.0.extract(),
        }
    }
}

impl From<(CdnToken, ExpireDateTime)> for CdnTokenPb {
    fn from(value: (CdnToken, ExpireDateTime)) -> Self {
        match value.0 {
            CdnToken::AWSCloudfront(token) => CdnTokenPb {
                expires: value.1.extract_to_timestamp(),
                aws_cloudfront_key_pair_id: token.key_pair_id,
                aws_cloudfront_signature: token.signature,
                aws_cloudfront_policy: token.policy,
            },
        }
    }
}

impl Into<Option<AuthToken>> for AuthTokenPb {
    fn into(self) -> Option<AuthToken> {
        match (
            self.authenticate_token,
            self.authorize_token,
            self.cdn_token,
        ) {
            (Some(authenticate_token), Some(authorize_token), Some(cdn_token)) => Some(AuthToken {
                authenticate_token: authenticate_token.into(),
                authorize_token: authorize_token.into(),
                cdn_token: cdn_token.into(),
            }),
            _ => None,
        }
    }
}

impl Into<(AuthenticateToken, ExpireDateTime)> for AuthenticateTokenPb {
    fn into(self) -> (AuthenticateToken, ExpireDateTime) {
        (
            AuthenticateToken::restore(self.token),
            ExpireDateTime::restore_from_timestamp(self.expires),
        )
    }
}

impl Into<(AuthorizeToken, ExpireDateTime)> for AuthorizeTokenPb {
    fn into(self) -> (AuthorizeToken, ExpireDateTime) {
        (
            AuthorizeToken::restore(self.token),
            ExpireDateTime::restore_from_timestamp(self.expires),
        )
    }
}

impl Into<(CdnToken, ExpireDateTime)> for CdnTokenPb {
    fn into(self) -> (CdnToken, ExpireDateTime) {
        (
            CdnToken::AWSCloudfront(AWSCloudfrontToken {
                key_pair_id: self.aws_cloudfront_key_pair_id,
                signature: self.aws_cloudfront_signature,
                policy: self.aws_cloudfront_policy,
            }),
            ExpireDateTime::restore_from_timestamp(self.expires),
        )
    }
}
