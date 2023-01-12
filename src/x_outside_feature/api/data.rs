use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct RequestId(String);

impl RequestId {
    pub fn generate() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn restore(value: String) -> Self {
        Self(value)
    }

    pub fn not_specified() -> Self {
        Self("(no request-id)".to_owned())
    }

    pub fn extract(self) -> String {
        self.0
    }
}
