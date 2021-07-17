pub trait Logger {
    fn error(&self, message: impl LogMessage);
    fn audit(&self, message: impl LogMessage);
    fn info(&self, message: impl LogMessage);
    fn debug(&self, message: impl LogMessage);

    fn log(&self, log_level: LogLevel, message: impl LogMessage) {
        match log_level {
            LogLevel::Error => self.error(message),
            LogLevel::Audit => self.audit(message),
            LogLevel::Info => self.info(message),
            LogLevel::Debug => self.debug(message),
        }
    }
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
