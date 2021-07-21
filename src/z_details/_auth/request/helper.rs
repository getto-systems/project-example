use tonic::metadata::MetadataMap;

use super::data::MetadataError;

pub fn metadata(metadata: &MetadataMap, key: &str) -> Result<String, MetadataError> {
    metadata
        .get(key)
        .ok_or(MetadataError::NotFound)
        .and_then(|value| {
            value
                .to_str()
                .map_err(|err| MetadataError::Invalid(format!("{}", err)))
        })
        .map(|value| value.into())
}
