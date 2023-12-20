use tonic::metadata::MetadataMap;

use crate::common::api::request::x_tonic::metadata::metadata;

use crate::x_content::metadata::{METADATA_METHOD, METADATA_PATH, METADATA_REQUEST_ID};

use crate::common::api::request::data::{RequestId, RequestInfo};

impl RequestInfo {
    pub fn from_metadata(map: &MetadataMap) -> Self {
        let mut info = Self::default();

        if let Ok(Some(value)) = metadata(map, METADATA_REQUEST_ID) {
            info.id = value.to_owned();
        }
        if let Ok(Some(value)) = metadata(map, METADATA_PATH) {
            info.path = value.to_owned();
        }
        if let Ok(Some(value)) = metadata(map, METADATA_METHOD) {
            info.method = value.to_owned();
        }

        info
    }
}

impl RequestId {
    pub fn from_metadata(map: &MetadataMap) -> RequestId {
        match metadata(map, METADATA_REQUEST_ID) {
            Ok(Some(request_id)) => RequestId::restore(request_id.to_owned()),
            _ => RequestId::not_specified(),
        }
    }
}
