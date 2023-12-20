use crate::avail::unexpected_error::notify::data::NotifyUnexpectedError;

pub trait NotifyUnexpectedErrorFieldsExtract {
    fn convert(self) -> NotifyUnexpectedError;
}

impl NotifyUnexpectedErrorFieldsExtract for NotifyUnexpectedError {
    fn convert(self) -> NotifyUnexpectedError {
        self
    }
}

pub trait NotifyUnexpectedErrorInfra {
    type Notifier: NotifyUnexpectedErrorNotifier;

    fn notifier(&self) -> &Self::Notifier;
}

pub trait NotifyUnexpectedErrorNotifier {
    fn notify(&self, err: NotifyUnexpectedError) -> NotifyUnexpectedError;
}

pub trait NotifyUnexpectedErrorLogger: Send + Sync {
    fn unexpected_error_occurred(&self, err: NotifyUnexpectedError) -> NotifyUnexpectedError;
}
