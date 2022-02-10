pub trait Logger {
    fn log(&self, message: &(impl LogFilter + LogMessage));
}
pub trait LogFilter {
    fn log_level(&self) -> LogLevel;
}
pub trait LogMessage {
    fn log_message(&self) -> String;
}

pub enum LogLevel {
    Error,
    Audit,
    Info,
    Debug,
}
