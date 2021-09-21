use actix_web::{HttpRequest, HttpResponse};

use crate::z_details::_common::response::actix_web::RespondTo;

use crate::auth::_api::proxy::AuthProxyResponseEncoder;

use crate::z_details::_api::message::data::MessageError;

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
