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
        AuthTokenResponse,
        AuthTokenMessage, // ValidateAuthNonceError, ValidateAuthRolesError,
    },
    _common::kernel::data::{AuthTokenEncoded, AuthTokenExtract, CloudfrontTokenKind},
};

impl RespondTo for AuthTokenResponse {
    fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        let AuthTokenResponse {
            domain,
            message: AuthTokenMessage { body: message, token },
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
