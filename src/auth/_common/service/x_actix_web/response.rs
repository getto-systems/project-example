use actix_web::{HttpRequest, HttpResponse};

use crate::auth::_common::service::x_actix_web::header::{
    COOKIE_API_TOKEN, COOKIE_CLOUDFRONT_KEY_PAIR_ID, COOKIE_CLOUDFRONT_POLICY,
    COOKIE_CLOUDFRONT_SIGNATURE, COOKIE_TICKET_TOKEN,
};

use crate::z_details::_common::response::actix_web::RespondTo;

use crate::auth::_common::service::data::AuthServiceError;

impl RespondTo for AuthServiceError {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::InvalidArgument(_) => HttpResponse::BadRequest().finish(),
            Self::AlreadyExists(_) => HttpResponse::Conflict().finish(),
            Self::Unauthenticated(_) => unauthorized(request),
            Self::PermissionDenied(_) => HttpResponse::Unauthorized().finish(),
            Self::Internal(_) => HttpResponse::InternalServerError().finish(),
            Self::Cancelled(_) => HttpResponse::Accepted().finish(),
            Self::InfraError(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}

fn unauthorized(request: &HttpRequest) -> HttpResponse {
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
