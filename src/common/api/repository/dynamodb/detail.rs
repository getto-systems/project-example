use std::collections::HashMap;

use aws_sdk_dynamodb::types::AttributeValue;

pub trait DynamoDbColumn {
    type Value;

    fn as_name() -> &'static str;
    fn into(value: Self::Value) -> AttributeValue;
    fn from(attr: AttributeValue) -> Option<Self::Value>;

    fn into_attr(value: Self::Value) -> (String, AttributeValue) {
        Self::into_attr_with_name(Self::as_name(), value)
    }
    fn into_attr_with_name(
        name: impl Into<String>,
        value: Self::Value,
    ) -> (String, AttributeValue) {
        (name.into(), Self::into(value))
    }
    fn restore(attrs: &mut HashMap<String, AttributeValue>) -> Option<Self::Value> {
        attrs
            .remove(Self::as_name())
            .and_then(|attr| Self::from(attr))
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
            Self::Last => false,
            _ => true,
        }
    }
    pub fn extract(self) -> Option<HashMap<String, AttributeValue>> {
        match self {
            Self::Next(key) => Some(key),
            _ => None,
        }
    }
}
