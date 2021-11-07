use actix_web::HttpRequest;

use crate::z_lib::remote::request::data::MetadataError;

pub fn header(request: &HttpRequest, key: &str) -> Result<Option<String>, MetadataError> {
    let header = request.headers().get(key);
    match header {
        None => Ok(None),
        Some(header) => header
            .to_str()
            .map(|value| Some(value.into()))
            .map_err(|err| MetadataError::Invalid(format!("{}", err))),
    }
}
pub fn cookie(request: &HttpRequest, key: &str) -> Option<String> {
    request.cookie(key).map(|cookie| cookie.value().to_string())
}
