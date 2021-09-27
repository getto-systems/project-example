use actix_web::{HttpRequest, HttpResponse};

use crate::z_details::_common::{logger::LogLevel, response::actix_web::RespondTo};

use crate::{
    auth::auth_ticket::remote::kernel::data::DecodeAuthTokenError,
    z_details::_common::request::data::MetadataError,
};

pub enum ValidateAuthMetadataError {
    MetadataError(MetadataError),
    DecodeError(DecodeAuthTokenError),
}

impl std::fmt::Display for ValidateAuthMetadataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MetadataError(err) => err.fmt(f),
            Self::DecodeError(err) => err.fmt(f),
        }
    }
}

impl RespondTo for ValidateAuthMetadataError {
    fn respond_to(self, request: &HttpRequest) -> HttpResponse {
        match self {
            Self::MetadataError(err) => err.respond_to(request),
            Self::DecodeError(err) => err.respond_to(request),
        }
    }
}

impl ValidateAuthMetadataError {
    pub fn log_level(&self) -> LogLevel {
        match self {
            Self::MetadataError(err) => err.log_level(),
            Self::DecodeError(err) => err.log_level(),
        }
    }
}
