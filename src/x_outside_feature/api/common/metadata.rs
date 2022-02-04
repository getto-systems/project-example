use tonic::metadata::MetadataMap;

use crate::z_lib::api::request::x_tonic::metadata::metadata;

pub const METADATA_REQUEST_ID: &'static str = "getto-example-request-id";

pub fn metadata_request_id(map: &MetadataMap) -> &str {
    match metadata(map, METADATA_REQUEST_ID) {
        Ok(Some(request_id)) => request_id,
        _ => "(no request-id)",
    }
}
