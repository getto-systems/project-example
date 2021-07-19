use actix_web::{HttpRequest, HttpResponse};

pub trait RespondTo {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse;
}
