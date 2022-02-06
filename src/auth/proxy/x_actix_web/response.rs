use actix_web::HttpResponse;

use crate::auth::ticket::kernel::api::x_actix_web::header::{
    COOKIE_API_TOKEN, COOKIE_CLOUDFRONT_KEY_PAIR_ID, COOKIE_CLOUDFRONT_POLICY,
    COOKIE_CLOUDFRONT_SIGNATURE, COOKIE_TICKET_TOKEN,
};

use crate::z_lib::api::response::actix_web::ProxyResponder;

use crate::auth::proxy::action::AuthProxyState;

use crate::auth::proxy::data::{AuthProxyError, AuthProxyResponse};

impl<R: ProxyResponder> ProxyResponder for AuthProxyState<R> {
    fn respond_to(self) -> HttpResponse {
        match self {
            Self::Metadata(event) => event.respond_to(),
            Self::TryToCall(_) => HttpResponse::Accepted().finish(),
            Self::Response(response) => response.respond_to(),
            Self::ServiceError(err) => err.respond_to(),
        }
    }
}

impl ProxyResponder for AuthProxyResponse {
    fn respond_to(self) -> HttpResponse {
        HttpResponse::Ok().body(self.extract())
    }
}

impl ProxyResponder for AuthProxyError {
    fn respond_to(self) -> HttpResponse {
        match self {
            Self::AlreadyExists(_) => HttpResponse::Conflict().finish(),
            Self::Unauthenticated(_) => unauthorized(),
            Self::PermissionDenied(_) => HttpResponse::Unauthorized().finish(),
            Self::Cancelled(_) => HttpResponse::Accepted().finish(),
            Self::InfraError(_) => HttpResponse::InternalServerError().finish(),
            Self::MessageError(err) => err.respond_to(),
        }
    }
}

pub fn unauthorized() -> HttpResponse {
    let mut response = HttpResponse::Unauthorized().finish();

    response.del_cookie(COOKIE_TICKET_TOKEN);
    response.del_cookie(COOKIE_API_TOKEN);
    response.del_cookie(COOKIE_CLOUDFRONT_SIGNATURE);
    response.del_cookie(COOKIE_CLOUDFRONT_KEY_PAIR_ID);
    response.del_cookie(COOKIE_CLOUDFRONT_POLICY);

    response
}
