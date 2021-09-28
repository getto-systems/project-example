use actix_web::{HttpRequest, HttpResponse};

use crate::z_lib::remote::response::actix_web::RespondTo;

use crate::auth::remote::service::proxy::AuthProxyResponseEncoder;

use crate::z_lib::remote::message::data::MessageError;

pub struct ResponseEncoder;

impl AuthProxyResponseEncoder<(), LogoutProxyResponse> for ResponseEncoder {
    fn encode(&self, _response: ()) -> Result<LogoutProxyResponse, MessageError> {
        Ok(LogoutProxyResponse)
    }
}

pub struct LogoutProxyResponse;

impl RespondTo for LogoutProxyResponse {
    fn respond_to(self, _request: &HttpRequest) -> HttpResponse {
        HttpResponse::Ok().finish()
    }
}
