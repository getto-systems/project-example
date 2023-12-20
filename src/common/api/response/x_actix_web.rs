use actix_web::HttpResponse;

pub trait ProxyResponder {
    fn respond_to(self) -> HttpResponse;
}

impl<T: ProxyResponder, E: ProxyResponder> ProxyResponder for Result<T, E> {
    fn respond_to(self) -> HttpResponse {
        match self {
            Ok(result) => result.respond_to(),
            Err(err) => err.respond_to(),
        }
    }
}
