use actix_web::{
    cookie::{Cookie, Expiration, SameSite},
    HttpResponse, HttpResponseBuilder,
};
use time::OffsetDateTime;

use crate::{
    auth::ticket::kernel::data::AuthPermissionError,
    x_content::metadata::{
        COOKIE_AUTHENTICATE_TOKEN, COOKIE_AUTHORIZE_TOKEN, COOKIE_CLOUDFRONT_KEY_PAIR_ID,
        COOKIE_CLOUDFRONT_POLICY, COOKIE_CLOUDFRONT_SIGNATURE,
    },
};

use crate::common::api::response::actix_web::ProxyResponder;

use crate::auth::{
    kernel::data::ExpireDateTime,
    proxy::data::{AuthProxyError, ProxyDomain},
    ticket::{
        authenticate::proxy::data::ProxyResponseAuthenticated,
        kernel::data::{
            AuthenticateToken, AuthorizeToken, CdnToken, DecodeAuthenticateTokenError,
            DecodeAuthorizeTokenError, ValidateAuthenticateTokenError,
        },
    },
};

impl ProxyResponder for ProxyResponseAuthenticated {
    fn respond_to(self) -> HttpResponse {
        match self.1 {
            None => delete_credentials(HttpResponse::Ok().body(self.0.extract())),
            Some((token, domain)) => {
                let mut response = HttpResponse::Ok();

                add_authenticate_cookie(&mut response, token.authenticate_token, domain.clone());
                add_authorize_cookie(&mut response, token.authorize_token, domain.clone());
                add_cdn_cookie(&mut response, token.cdn_token, domain);

                response.body(self.0.extract())
            }
        }
    }
}

fn add_authenticate_cookie(
    response: &mut HttpResponseBuilder,
    token: (AuthenticateToken, ExpireDateTime),
    domain: ProxyDomain,
) {
    if let Ok(offset) = OffsetDateTime::from_unix_timestamp(token.1.extract_to_timestamp()) {
        response.cookie(
            Cookie::build(COOKIE_AUTHENTICATE_TOKEN, token.0.extract())
                .expires(Expiration::DateTime(offset))
                .domain(domain.extract())
                .path("/")
                .secure(true)
                .http_only(true)
                .same_site(SameSite::Strict)
                .finish(),
        );
    }
}

fn add_authorize_cookie(
    response: &mut HttpResponseBuilder,
    token: (AuthorizeToken, ExpireDateTime),
    domain: ProxyDomain,
) {
    if let Ok(offset) = OffsetDateTime::from_unix_timestamp(token.1.extract_to_timestamp()) {
        response.cookie(
            Cookie::build(COOKIE_AUTHORIZE_TOKEN, token.0.extract())
                .expires(Expiration::DateTime(offset))
                .domain(domain.extract())
                .path("/")
                .secure(true)
                .http_only(true)
                .same_site(SameSite::Strict)
                .finish(),
        );
    }
}

fn add_cdn_cookie(
    response: &mut HttpResponseBuilder,
    token: (CdnToken, ExpireDateTime),
    domain: ProxyDomain,
) {
    if let Ok(offset) = OffsetDateTime::from_unix_timestamp(token.1.extract_to_timestamp()) {
        match token.0 {
            CdnToken::AWSCloudfront(token) => {
                response.cookie(
                    Cookie::build(COOKIE_CLOUDFRONT_KEY_PAIR_ID, token.key_pair_id)
                        .expires(Expiration::DateTime(offset.clone()))
                        .domain(domain.clone().extract())
                        .path("/")
                        .secure(true)
                        .http_only(true)
                        .same_site(SameSite::Strict)
                        .finish(),
                );
                response.cookie(
                    Cookie::build(COOKIE_CLOUDFRONT_POLICY, token.policy)
                        .expires(Expiration::DateTime(offset.clone()))
                        .domain(domain.clone().extract())
                        .path("/")
                        .secure(true)
                        .http_only(true)
                        .same_site(SameSite::Strict)
                        .finish(),
                );
                response.cookie(
                    Cookie::build(COOKIE_CLOUDFRONT_SIGNATURE, token.signature)
                        .expires(Expiration::DateTime(offset))
                        .domain(domain.extract())
                        .path("/")
                        .secure(true)
                        .http_only(true)
                        .same_site(SameSite::Strict)
                        .finish(),
                );
            }
        }
    }
}

impl ProxyResponder for AuthProxyError {
    fn respond_to(self) -> HttpResponse {
        match self {
            Self::Unauthenticated(_) => unauthenticated(),
            Self::InfraError(_) => HttpResponse::InternalServerError().finish(),
            Self::ServiceConnectError(err) => err.respond_to(),
            Self::ServiceMetadataError(err) => err.respond_to(),
            Self::MessageError(err) => err.respond_to(),
        }
    }
}

impl ProxyResponder for ValidateAuthenticateTokenError {
    fn respond_to(self) -> HttpResponse {
        unauthenticated()
    }
}

impl ProxyResponder for DecodeAuthenticateTokenError {
    fn respond_to(self) -> HttpResponse {
        match self {
            Self::Expired => unauthenticated(),
            Self::Invalid(_) => unauthenticated(),
        }
    }
}

impl ProxyResponder for DecodeAuthorizeTokenError {
    fn respond_to(self) -> HttpResponse {
        match self {
            Self::Expired => unauthenticated(),
            Self::Invalid(_) => unauthenticated(),
        }
    }
}

impl ProxyResponder for AuthPermissionError {
    fn respond_to(self) -> HttpResponse {
        match self {
            Self::PermissionDenied(_, _) => unauthenticated(),
        }
    }
}

fn unauthenticated() -> HttpResponse {
    delete_credentials(HttpResponse::Unauthorized().finish())
}

fn delete_credentials(mut response: HttpResponse) -> HttpResponse {
    response.del_cookie(COOKIE_AUTHENTICATE_TOKEN);
    response.del_cookie(COOKIE_AUTHORIZE_TOKEN);
    response.del_cookie(COOKIE_CLOUDFRONT_KEY_PAIR_ID);
    response.del_cookie(COOKIE_CLOUDFRONT_POLICY);
    response.del_cookie(COOKIE_CLOUDFRONT_SIGNATURE);

    response
}
