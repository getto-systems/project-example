use crate::common::api::logger::infra::LogOutputLevel;

impl Default for LogOutputLevel {
    fn default() -> Self {
        Self::Verbose
    }
}
