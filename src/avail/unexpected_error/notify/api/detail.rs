use std::sync::Arc;

use crate::x_outside_feature::core::feature::CoreAppFeature;

use crate::{auth::feature::AsAuthorizedInfra, common::api::logger::detail::StdoutJsonLogger};

use crate::avail::unexpected_error::notify::action::NotifyUnexpectedErrorInfo;

use crate::avail::unexpected_error::notify::infra::{
    NotifyUnexpectedErrorInfra, NotifyUnexpectedErrorLogger, NotifyUnexpectedErrorNotifier,
};

use crate::{
    auth::data::{AuthPermissionRequired, AuthorizeSuccess},
    avail::unexpected_error::notify::data::NotifyUnexpectedError,
};

pub struct LiveNotifyUnexpectedErrorInfra {
    notifier: NoopNotifyUnexpectedErrorNotifier,
}

impl AsAuthorizedInfra<LiveNotifyUnexpectedErrorInfra> for Arc<CoreAppFeature> {
    fn required(&self) -> AuthPermissionRequired {
        NotifyUnexpectedErrorInfo::required()
    }
    fn as_authorized_infra(&self, _: &AuthorizeSuccess) -> LiveNotifyUnexpectedErrorInfra {
        LiveNotifyUnexpectedErrorInfra {
            notifier: NoopNotifyUnexpectedErrorNotifier,
        }
    }
}

impl NotifyUnexpectedErrorInfra for LiveNotifyUnexpectedErrorInfra {
    type Notifier = NoopNotifyUnexpectedErrorNotifier;

    fn notifier(&self) -> &Self::Notifier {
        &self.notifier
    }
}

pub struct NoopNotifyUnexpectedErrorNotifier;

#[async_trait::async_trait]
impl NotifyUnexpectedErrorNotifier for NoopNotifyUnexpectedErrorNotifier {
    fn notify(&self, err: NotifyUnexpectedError) -> NotifyUnexpectedError {
        err
    }
}

impl NotifyUnexpectedErrorLogger for StdoutJsonLogger {
    fn unexpected_error_occurred(&self, err: NotifyUnexpectedError) -> NotifyUnexpectedError {
        self.fatal(format!("{}", err));
        err
    }
}

#[cfg(test)]
pub mod test {
    use crate::{
        avail::unexpected_error::notify::api::detail::NoopNotifyUnexpectedErrorNotifier,
        common::api::feature::AsInfra,
    };

    use crate::avail::unexpected_error::notify::infra::NotifyUnexpectedErrorInfra;

    pub struct MockNotifyUnexpectedErrorInfra {
        notifier: NoopNotifyUnexpectedErrorNotifier,
    }

    impl AsInfra<MockNotifyUnexpectedErrorInfra> for () {
        fn as_infra(&self) -> MockNotifyUnexpectedErrorInfra {
            MockNotifyUnexpectedErrorInfra {
                notifier: NoopNotifyUnexpectedErrorNotifier,
            }
        }
    }

    impl NotifyUnexpectedErrorInfra for MockNotifyUnexpectedErrorInfra {
        type Notifier = NoopNotifyUnexpectedErrorNotifier;

        fn notifier(&self) -> &Self::Notifier {
            &self.notifier
        }
    }
}
