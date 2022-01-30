use actix_web::{HttpRequest, HttpResponse};

use crate::auth::ticket::remote::kernel::x_actix_web::header::{
    COOKIE_API_TOKEN, COOKIE_CLOUDFRONT_KEY_PAIR_ID, COOKIE_CLOUDFRONT_POLICY,
    COOKIE_CLOUDFRONT_SIGNATURE, COOKIE_TICKET_TOKEN,
};

use crate::z_lib::remote::response::actix_web::RespondTo;

use crate::auth::remote::proxy::action::AuthProxyState;

use crate::auth::remote::proxy::data::{AuthProxyError, AuthProxyResponse};

impl<R: RespondTo, E: RespondTo> RespondTo for AuthProxyState<R, E> {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Metadata(event) => event.respond_to(request),
            Self::TryToCall(_) => HttpResponse::Accepted().finish(),
            Self::Response(response) => response.respond_to(request),
            Self::ServiceError(err) => err.respond_to(request),
        }
    }
}

impl RespondTo for AuthProxyResponse {
    fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        HttpResponse::Ok().body(self.extract())
    }
}

impl RespondTo for AuthProxyError {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::InvalidArgument(_) => HttpResponse::BadRequest().finish(),
            Self::AlreadyExists(_) => HttpResponse::Conflict().finish(),
            Self::Unauthenticated(_) => unauthorized(request),
            Self::PermissionDenied(_) => HttpResponse::Unauthorized().finish(),
            Self::Internal(_) => HttpResponse::InternalServerError().finish(),
            Self::Cancelled(_) => HttpResponse::Accepted().finish(),
            Self::InfraError(_) => HttpResponse::InternalServerError().finish(),
            Self::MessageError(err) => err.respond_to(request),
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
