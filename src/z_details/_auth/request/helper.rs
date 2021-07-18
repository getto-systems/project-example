use tonic::Request;

use super::data::MetadataError;

pub fn metadata<T>(request: &Request<T>, key: &str) -> Result<String, MetadataError> {
    request
        .metadata()
        .get(key)
        .ok_or(MetadataError::NotFound)
        .and_then(|value| {
            value
                .to_str()
                .map_err(|err| MetadataError::Invalid(format!("{}", err)))
        })
        .map(|value| value.into())
}
