use actix_web::{
    cookie::{Cookie, SameSite},
    HttpRequest, HttpResponse,
};
use time::OffsetDateTime;

use crate::z_details::_common::response::actix_web::RespondTo;

use super::header::{
    COOKIE_API_TOKEN, COOKIE_CLOUDFRONT_KEY_PAIR_ID, COOKIE_CLOUDFRONT_POLICY,
    COOKIE_CLOUDFRONT_SIGNATURE, COOKIE_TICKET_TOKEN,
};

use crate::auth::auth_ticket::{
    _api::kernel::data::{
        AuthTokenMessage, AuthTokenMessageEncoded, ValidateAuthNonceError, ValidateAuthRolesError,
    },
    _common::kernel::data::{AuthTokenEncoded, AuthTokenExtract, CloudfrontTokenKind},
};

impl RespondTo for AuthTokenMessage {
    fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        let AuthTokenMessage {
            domain,
            message: AuthTokenMessageEncoded { message, token },
        } = self;
        let AuthTokenEncoded {
            ticket_token,
            api_token,
            cloudfront_tokens,
        } = token;

        let mut response = HttpResponse::Ok();

        response.cookie(auth_cookie(COOKIE_TICKET_TOKEN, &domain, ticket_token));
        response.cookie(auth_cookie(COOKIE_API_TOKEN, &domain, api_token));
        cloudfront_tokens
            .into_iter()
            .for_each(|(kind, cloudfront_token)| {
                response.cookie(auth_cookie(kind_as_name(&kind), &domain, cloudfront_token));
            });

        response.body(message)
    }
}

fn auth_cookie<'a>(name: &'a str, domain: &'a str, token: AuthTokenExtract) -> Cookie<'a> {
    Cookie::build(name, token.token)
        .expires(OffsetDateTime::from_unix_timestamp(token.expires))
        .domain(domain)
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Strict)
        .finish()
}

fn kind_as_name(kind: &CloudfrontTokenKind) -> &str {
    match kind {
        CloudfrontTokenKind::KeyPairId => COOKIE_CLOUDFRONT_KEY_PAIR_ID,
        CloudfrontTokenKind::Policy => COOKIE_CLOUDFRONT_POLICY,
        CloudfrontTokenKind::Signature => COOKIE_CLOUDFRONT_SIGNATURE,
    }
}

impl RespondTo for ValidateAuthNonceError {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::HeaderError(err) => err.respond_to(request),
            Self::RepositoryError(err) => err.respond_to(request),
            Self::Conflict => HttpResponse::Conflict().finish(),
        }
    }
}

impl RespondTo for ValidateAuthRolesError {
    fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        match self {
            Self::PermissionDenied(_, _) => HttpResponse::Unauthorized().finish(),
        }
    }
}

pub fn unauthorized(request: &HttpRequest) -> HttpResponse {
    let mut response = HttpResponse::Unauthorized();

    if let Some(cookie) = request.cookie(COOKIE_TICKET_TOKEN) {
        response.del_cookie(&cookie);
    }
    if let Some(cookie) = request.cookie(COOKIE_API_TOKEN) {
        response.del_cookie(&cookie);
    }
    if let Some(cookie) = request.cookie(COOKIE_CLOUDFRONT_SIGNATURE) {
        response.del_cookie(&cookie);
    }
    if let Some(cookie) = request.cookie(COOKIE_CLOUDFRONT_KEY_PAIR_ID) {
        response.del_cookie(&cookie);
    }
    if let Some(cookie) = request.cookie(COOKIE_CLOUDFRONT_POLICY) {
        response.del_cookie(&cookie);
    }

    response.finish()
}
