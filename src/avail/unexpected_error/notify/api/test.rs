use crate::common::api::feature::AsInfra;

use crate::avail::unexpected_error::notify::action::{
    NotifyUnexpectedErrorAction, NotifyUnexpectedErrorInfo,
};

use crate::{
    auth::data::AuthPermissionRequired,
    avail::unexpected_error::notify::data::NotifyUnexpectedError,
};

#[tokio::test]
async fn info() {
    assert_eq!(
        NotifyUnexpectedErrorInfo::required(),
        AuthPermissionRequired::Nothing,
    );
}

#[tokio::test]
async fn success() {
    let feature = feature();
    let action = NotifyUnexpectedErrorAction::mock(feature.as_infra());

    let fields = NotifyUnexpectedError::new("error".to_owned());

    action.notify(fields).await;
}

fn feature() -> () {
    ()
}
