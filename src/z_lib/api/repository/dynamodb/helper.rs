use std::collections::HashMap;

use chrono::{DateTime, Utc};
use rusoto_dynamodb::AttributeValue;

pub trait DynamoDbColumn {
    type Value;

    fn as_name() -> &'static str;
    fn to_attr(value: Self::Value) -> AttributeValue;
    fn to_value(attr: AttributeValue) -> Option<Self::Value>;

    fn to_attr_pair(value: Self::Value) -> (String, AttributeValue) {
        (Self::as_name().into(), Self::to_attr(value))
    }
    fn remove_value(attrs: &mut HashMap<String, AttributeValue>) -> Option<Self::Value> {
        attrs
            .remove(Self::as_name())
            .and_then(|attr| Self::to_value(attr))
    }
}

pub fn string_value(value: String) -> AttributeValue {
    AttributeValue {
        s: Some(value),
        ..Default::default()
    }
}
pub fn string_set_value(value: Vec<String>) -> AttributeValue {
    AttributeValue {
        ss: Some(value),
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
