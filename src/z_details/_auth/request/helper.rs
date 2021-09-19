use tonic::metadata::MetadataMap;

use crate::z_details::_common::request::data::MetadataError;

pub fn metadata(metadata: &MetadataMap, key: &str) -> Result<Option<String>, MetadataError> {
    let value = metadata.get(key);
    match value {
        None => Ok(None),
        Some(value) => value
            .to_str()
            .map(|value| Some(value.to_string()))
            .map_err(|err| MetadataError::Invalid(format!("{}", err))),
    }
}
