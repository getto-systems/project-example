use crate::common::api::search::data::SearchLimit;

impl Default for SearchLimit {
    fn default() -> Self {
        match SearchLimit::convert(1000) {
            Ok(limit) => limit,
            Err(err) => panic!("failed to convert limit: {}", err),
        }
    }
}
