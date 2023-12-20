mod detail;

use std::sync::Arc;

use crate::avail::unexpected_error::notify::infra::{
    NotifyUnexpectedErrorFieldsExtract, NotifyUnexpectedErrorInfra, NotifyUnexpectedErrorLogger,
    NotifyUnexpectedErrorNotifier,
};

use crate::{
    auth::data::AuthPermissionRequired,
    avail::unexpected_error::notify::data::NotifyUnexpectedError,
};

pub struct NotifyUnexpectedErrorAction<M: NotifyUnexpectedErrorInfra> {
    infra: M,
    logger: Arc<dyn NotifyUnexpectedErrorLogger>,
}

pub struct NotifyUnexpectedErrorInfo;

impl NotifyUnexpectedErrorInfo {
    pub fn required() -> AuthPermissionRequired {
        AuthPermissionRequired::Nothing
    }
}

impl<M: NotifyUnexpectedErrorInfra> NotifyUnexpectedErrorAction<M> {
    pub async fn notify(
        &self,
        fields: impl NotifyUnexpectedErrorFieldsExtract,
    ) -> NotifyUnexpectedError {
        self.logger
            .unexpected_error_occurred(self.infra.notifier().notify(fields.convert()))
    }
}
