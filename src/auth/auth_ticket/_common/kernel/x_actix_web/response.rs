use actix_web::{HttpRequest, HttpResponse};

use crate::z_details::_common::response::actix_web::RespondTo;

use crate::auth::_common::service::x_actix_web::response::unauthorized;

use crate::auth::auth_ticket::_common::kernel::data::DecodeAuthTokenError;

impl RespondTo for DecodeAuthTokenError {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        unauthorized(request)
    }
}
