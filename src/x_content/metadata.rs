use tonic::metadata::MetadataMap;

use crate::z_lib::request::x_tonic::metadata::metadata;

pub const HEADER_NONCE: &'static str = "GETTO-EXAMPLE-NONCE";

pub const COOKIE_TICKET_TOKEN: &'static str = "__Secure-GETTO-EXAMPLE-TICKET-TOKEN";
pub const COOKIE_API_TOKEN: &'static str = "__Secure-GETTO-EXAMPLE-API-TOKEN";

pub const METADATA_NONCE: &'static str = "getto-example-nonce";
pub const METADATA_TOKEN: &'static str = "getto-example-token";
pub const METADATA_REQUEST_ID: &'static str = "getto-example-request-id";

pub const COOKIE_CLOUDFRONT_SIGNATURE: &'static str = "CloudFront-Signature";
pub const COOKIE_CLOUDFRONT_KEY_PAIR_ID: &'static str = "CloudFront-Key-Pair-Id";
pub const COOKIE_CLOUDFRONT_POLICY: &'static str = "CloudFront-Policy";

pub fn metadata_request_id(map: &MetadataMap) -> &str {
    match metadata(map, METADATA_REQUEST_ID) {
        Ok(Some(request_id)) => request_id,
        _ => "(no request-id)",
    }
}
