use actix_web::HttpResponse;

pub trait ProxyResponder {
    fn respond_to(self) -> HttpResponse;
}
