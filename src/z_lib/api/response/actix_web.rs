use actix_web::{HttpRequest, HttpResponse};

pub trait RespondTo {
    // TODO request は必要無くなった
    fn respond_to(self, request: &HttpRequest) -> HttpResponse;
}
