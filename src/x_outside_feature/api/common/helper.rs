use tonic::metadata::MetadataMap;

use crate::z_lib::request::x_tonic::metadata::metadata;

use crate::x_content::metadata::METADATA_REQUEST_ID;

pub fn metadata_request_id(map: &MetadataMap) -> &str {
    match metadata(map, METADATA_REQUEST_ID) {
        Ok(Some(request_id)) => request_id,
        _ => "(no request-id)",
    }
}
