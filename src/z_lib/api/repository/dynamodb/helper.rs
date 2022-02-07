use chrono::{DateTime, Utc};
use rusoto_dynamodb::AttributeValue;

pub fn string_value(value: String) -> AttributeValue {
    AttributeValue {
        s: Some(value),
        ..Default::default()
    }
}
pub fn timestamp_value(value: DateTime<Utc>) -> AttributeValue {
    AttributeValue {
        n: Some(value.timestamp().to_string()),
        ..Default::default()
    }
}
