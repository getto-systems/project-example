use tonic::metadata::MetadataMap;

use crate::z_lib::api::request::data::MetadataError;

pub fn metadata<'a>(
    metadata: &'a MetadataMap,
    key: &str,
) -> Result<Option<&'a str>, MetadataError> {
    let value = metadata.get(key);
    match value {
        None => Ok(None),
        Some(value) => value
            .to_str()
            .map(Some)
            .map_err(|err| MetadataError::Invalid(format!("{}", err))),
    }
}
