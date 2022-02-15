use std::collections::HashMap;

use chrono::{DateTime, Utc};
use rusoto_dynamodb::AttributeValue;

pub fn string_value(value: String) -> AttributeValue {
    AttributeValue {
        s: Some(value),
        ..Default::default()
    }
}
pub fn bool_value(value: bool) -> AttributeValue {
    AttributeValue {
        bool: Some(value),
        ..Default::default()
    }
}
pub fn timestamp_value(value: DateTime<Utc>) -> AttributeValue {
    AttributeValue {
        n: Some(value.timestamp().to_string()),
        ..Default::default()
    }
}

pub enum ScanKey {
    FirstTime,
    Next(HashMap<String, AttributeValue>),
    Last,
}
impl ScanKey {
    pub fn next(key: Option<HashMap<String, AttributeValue>>) -> Self {
        match key {
            Some(key) => Self::Next(key),
            None => Self::Last,
        }
    }

    pub fn has_next(&self) -> bool {
        match self {
            ScanKey::Last => false,
            _ => true,
        }
    }
    pub fn extract(self) -> Option<HashMap<String, AttributeValue>> {
        match self {
            ScanKey::Next(key) => Some(key),
            _ => None,
        }
    }
}
