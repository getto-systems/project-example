use actix_web::{HttpRequest, HttpResponse};

use crate::z_lib::remote::response::actix_web::RespondTo;

use crate::example::outline::remote::proxy_get_menu_badge::data::GetOutlineMenuBadgeProxyMessage;

impl RespondTo for GetOutlineMenuBadgeProxyMessage {
    fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        match self {
            Self::Success(message) => HttpResponse::Ok().body(message),
        }
    }
}
