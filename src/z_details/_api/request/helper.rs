use actix_web::{HttpMessage, HttpRequest};

use super::data::HeaderError;

pub fn header(request: &HttpRequest, key: &str) -> Result<String, HeaderError> {
    request
        .headers()
        .get(key)
        .ok_or(HeaderError::NotFound)
        .and_then(|header| {
            header
                .to_str()
                .map(String::from)
                .map_err(|err| HeaderError::Invalid(format!("{}", err)))
        })
}
pub fn cookie(request: &HttpRequest, key: &str) -> Result<String, HeaderError> {
    request
        .cookie(key)
        .map(|cookie| cookie.value().to_string())
        .ok_or(HeaderError::NotFound)
}
