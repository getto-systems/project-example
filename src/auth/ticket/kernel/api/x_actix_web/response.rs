use actix_web::{
    cookie::{Cookie, Expiration, SameSite},
    HttpRequest, HttpResponse,
};
use time::{error::ComponentRange, OffsetDateTime};

use crate::z_lib::api::response::actix_web::RespondTo;

use super::header::{
    COOKIE_API_TOKEN, COOKIE_CLOUDFRONT_KEY_PAIR_ID, COOKIE_CLOUDFRONT_POLICY,
    COOKIE_CLOUDFRONT_SIGNATURE, COOKIE_TICKET_TOKEN,
};

use crate::auth::proxy::x_actix_web::response::unauthorized;

use crate::auth::ticket::kernel::api::data::{
    AuthTokenExtract, AuthTokenMessage, AuthTokenResponse, CloudfrontTokenKind,
    DecodeAuthTokenError, EncodedAuthTokens,
};

impl RespondTo for AuthTokenResponse {
    fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        let AuthTokenResponse {
            domain,
            message: AuthTokenMessage { body, token },
        } = self;
        let EncodedAuthTokens {
            ticket_token,
            api_token,
            cloudfront_tokens,
        } = token;

        let mut response = HttpResponse::Ok();

        // これらがエラーの時は web アプリケーションが動かなくなる
        // TODO ログを出したい
        if let Ok(cookie) = auth_cookie(COOKIE_TICKET_TOKEN, &domain, ticket_token) {
            response.cookie(cookie);
        }
        if let Ok(cookie) = auth_cookie(COOKIE_API_TOKEN, &domain, api_token) {
            response.cookie(cookie);
        }
        cloudfront_tokens
            .into_iter()
            .for_each(|(kind, cloudfront_token)| {
                if let Ok(cookie) = auth_cookie(kind_as_name(&kind), &domain, cloudfront_token) {
                    response.cookie(cookie);
                }
            });

        response.body(body)
    }
}

fn auth_cookie<'a>(
    name: &'a str,
    domain: &'a str,
    token: AuthTokenExtract,
) -> Result<Cookie<'a>, ComponentRange> {
    let expiration = Expiration::DateTime(OffsetDateTime::from_unix_timestamp(token.expires)?);
    Ok(Cookie::build(name, token.token)
        .expires(expiration)
        .domain(domain)
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Strict)
        .finish())
}

fn kind_as_name(kind: &CloudfrontTokenKind) -> &str {
    match kind {
        CloudfrontTokenKind::KeyPairId => COOKIE_CLOUDFRONT_KEY_PAIR_ID,
        CloudfrontTokenKind::Policy => COOKIE_CLOUDFRONT_POLICY,
        CloudfrontTokenKind::Signature => COOKIE_CLOUDFRONT_SIGNATURE,
    }
}

impl RespondTo for DecodeAuthTokenError {
    fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        unauthorized()
    }
}
