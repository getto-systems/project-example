use actix_web::{HttpMessage, HttpRequest, HttpResponse};

use super::header::{
    COOKIE_API_TOKEN, COOKIE_CLOUDFRONT_KEY_PAIR_ID, COOKIE_CLOUDFRONT_POLICY,
    COOKIE_CLOUDFRONT_SIGNATURE, COOKIE_TICKET_TOKEN,
};

use super::super::data::{ValidateAuthNonceError, ValidateAuthRolesError};

impl ValidateAuthNonceError {
    pub fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::HeaderError(err) => err.respond_to(request),
            Self::RepositoryError(err) => err.respond_to(request),
            Self::Conflict => HttpResponse::Conflict().finish(),
        }
    }
}

impl ValidateAuthRolesError {
    pub fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
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
