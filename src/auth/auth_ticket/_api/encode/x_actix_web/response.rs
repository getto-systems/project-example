use actix_web::{
    cookie::{Cookie, SameSite},
    HttpRequest, HttpResponse,
};
use time::OffsetDateTime;

use super::super::super::kernel::x_actix_web::response::unauthorized;

use super::super::event::EncodeAuthTicketEvent;

use super::super::super::kernel::data::ExpireDateTime;
use super::super::data::{AuthTokenEncoded, AuthTokenEncodedData, EncodeAuthTokenError};

impl EncodeAuthTicketEvent {
    pub fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::TokenExpiresCalculated(_) => HttpResponse::Accepted().finish(),
            Self::Success(token) => token.respond_to(request),
            Self::TicketNotFound => unauthorized(request),
            Self::RepositoryError(err) => err.respond_to(request),
            Self::EncodeError(err) => err.respond_to(request),
            Self::MessageError(err) => err.respond_to(request),
        }
    }
}

impl AuthTokenEncoded {
    fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        let mut response = HttpResponse::Ok();

        self.ticket_tokens.into_iter().for_each(|info| {
            response.cookie(auth_cookie(info));
        });
        self.api_tokens.into_iter().for_each(|info| {
            response.cookie(auth_cookie(info));
        });
        self.cdn_tokens.into_iter().for_each(|info| {
            response.cookie(auth_cookie(info));
        });

        response.body(self.message)
    }
}

fn auth_cookie<'a>(info: AuthTokenEncodedData) -> Cookie<'a> {
    Cookie::build(info.name, info.token.value)
        .expires(into_offset_date_time(info.token.expires))
        .domain(info.domain)
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Strict)
        .finish()
}

fn into_offset_date_time(src: ExpireDateTime) -> OffsetDateTime {
    OffsetDateTime::from_unix_timestamp(src.timestamp())
}

impl EncodeAuthTokenError {
    fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        match self {
            Self::InfraError(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}
