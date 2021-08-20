use actix_web::{HttpRequest, HttpResponse};

use crate::z_details::_common::response::actix_web::RespondTo;

use super::super::action::GetOutlineMenuBadgeState;

impl RespondTo for GetOutlineMenuBadgeState {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::GetMenuBadge(event) => event.respond_to(request),
        }
    }
}
