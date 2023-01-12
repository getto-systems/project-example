use getto_application_test::ApplicationActionStateHolder;
use pretty_assertions::assert_eq;

use crate::{
    auth::init::test::{StaticAuthorizeInfra, StaticAuthorizeToken},
    avail::unexpected_error::notify::init::test::{
        StaticNotifyUnexpectedErrorFields, StaticNotifyUnexpectedErrorMaterial,
    },
};

use crate::avail::unexpected_error::notify::action::NotifyUnexpectedErrorAction;

use crate::avail::unexpected_error::notify::infra::NotifyUnexpectedErrorFields;

#[tokio::test]
async fn info() {
    let material = StaticNotifyUnexpectedErrorMaterial {
        authorize: StaticAuthorizeInfra::standard(),
    };

    let action = NotifyUnexpectedErrorAction::with_material(material);

    let (name, required) = action.info.params();
    assert_eq!(
        format!("{}; {}", name, required),
        "avail.unexpected-error.notify; require: nothing",
    );
}

#[tokio::test]
async fn success_notify() {
    let holder = ApplicationActionStateHolder::new();

    let material = StaticNotifyUnexpectedErrorMaterial {
        authorize: StaticAuthorizeInfra::standard(),
    };
    let fields = StaticNotifyUnexpectedErrorFields {
        fields: NotifyUnexpectedErrorFields {
            err: "UNEXPECTED-ERROR".into(),
        },
    };

    let mut action = NotifyUnexpectedErrorAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(StaticAuthorizeToken, fields).await;
    assert_eq!(
        holder.extract(),
        vec![
            "try to proxy call: auth.ticket.authorize.clarify(require: nothing)",
            "proxy call success",
            "UNEXPECTED-ERROR",
        ],
    );
    assert!(result.is_ok());
}
