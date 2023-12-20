pub struct NotifyUnexpectedError(String);

impl NotifyUnexpectedError {
    pub fn new(err: String) -> Self {
        Self(err)
    }
}

impl std::fmt::Display for NotifyUnexpectedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
