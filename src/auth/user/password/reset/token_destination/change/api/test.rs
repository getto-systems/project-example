use getto_application_test::ApplicationActionStateHolder;
use pretty_assertions::assert_eq;

use crate::auth::{
    ticket::{
        authorize::init::test::StaticAuthorizeInfra,
        kernel::init::request::test::StaticAuthorizeToken,
    },
    user::{
        kernel::init::user_repository::memory::{MemoryAuthUserRepository, MemoryAuthUserStore},
        password::reset::token_destination::change::init::test::{
            StaticChangeResetTokenDestinationFields, StaticChangeResetTokenDestinationMaterial,
        },
    },
};

use crate::auth::user::password::reset::token_destination::change::action::ChangeResetTokenDestinationAction;

use crate::auth::user::password::reset::token_destination::change::infra::ChangeResetTokenDestinationFields;

use crate::{
    auth::user::{
        kernel::data::AuthUserId,
        login_id::kernel::data::LoginId,
        password::reset::{
            kernel::data::{
                ResetPasswordTokenDestination, ResetPasswordTokenDestinationEmail,
                ValidateResetPasswordTokenDestinationError,
            },
            token_destination::change::data::ValidateChangeResetTokenDestinationFieldsError,
        },
    },
    common::api::validate::data::ValidateTextError,
};

#[tokio::test]
async fn info() {
    let store = TestStore::new();
    let material = StaticChangeResetTokenDestinationMaterial {
        authorize: StaticAuthorizeInfra::new(operator_user_id()),
        destination_repository: standard_destination_repository(&store),
    };

    let action = ChangeResetTokenDestinationAction::with_material(material);

    let (name, required) = action.info.params();
    assert_eq!(
        format!("{}; {}", name, required),
        "auth.user.password.reset.token-destination.change; require: some [auth-user]",
    );
}

#[tokio::test]
async fn success_change_destination() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::new();
    let material = StaticChangeResetTokenDestinationMaterial {
        authorize: StaticAuthorizeInfra::new(operator_user_id()),
        destination_repository: standard_destination_repository(&store),
    };
    let fields =
        StaticChangeResetTokenDestinationFields::Valid(ChangeResetTokenDestinationFields {
            login_id: stored_login_id(),
            from: stored_destination(),
            to: ResetPasswordTokenDestination::Email(ResetPasswordTokenDestinationEmail::restore(
                "new-destination@example.com".to_owned(),
            )),
        });

    let mut action = ChangeResetTokenDestinationAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(StaticAuthorizeToken, fields).await;
    assert_eq!(
        holder.extract(),
        vec![
            "try to proxy call: auth.ticket.authorize.clarify(require: some [auth-user])",
            "proxy call success",
            "change reset token destination success",
        ]
    );
    assert!(result.is_ok());
}

#[tokio::test]
async fn error_conflict_changes() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::new();
    let material = StaticChangeResetTokenDestinationMaterial {
        authorize: StaticAuthorizeInfra::new(operator_user_id()),
        destination_repository: standard_destination_repository(&store),
    };
    let fields =
        StaticChangeResetTokenDestinationFields::Valid(ChangeResetTokenDestinationFields {
            login_id: stored_login_id(),
            from: ResetPasswordTokenDestination::Email(
                ResetPasswordTokenDestinationEmail::restore("UNKNOWN@example.com".to_owned()),
            ),
            to: ResetPasswordTokenDestination::Email(ResetPasswordTokenDestinationEmail::restore(
                "new-destination@example.com".to_owned(),
            )),
        });

    let mut action = ChangeResetTokenDestinationAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(StaticAuthorizeToken, fields).await;
    assert_eq!(
        holder.extract(),
        vec![
            "try to proxy call: auth.ticket.authorize.clarify(require: some [auth-user])",
            "proxy call success",
            "change reset token destination error; changes conflicted",
        ]
    );
    assert!(result.is_err());
}

#[tokio::test]
async fn error_not_found() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::new();
    let material = StaticChangeResetTokenDestinationMaterial {
        authorize: StaticAuthorizeInfra::new(operator_user_id()),
        destination_repository: no_destination_repository(&store),
    };
    let fields =
        StaticChangeResetTokenDestinationFields::Valid(ChangeResetTokenDestinationFields {
            login_id: stored_login_id(),
            from: stored_destination(),
            to: ResetPasswordTokenDestination::Email(ResetPasswordTokenDestinationEmail::restore(
                "new-destination@example.com".to_owned(),
            )),
        });

    let mut action = ChangeResetTokenDestinationAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(StaticAuthorizeToken, fields).await;
    assert_eq!(
        holder.extract(),
        vec![
            "try to proxy call: auth.ticket.authorize.clarify(require: some [auth-user])",
            "proxy call success",
            "change reset token destination error; not found",
        ]
    );
    assert!(result.is_err());
}

#[tokio::test]
async fn error_invalid_email() {
    let holder = ApplicationActionStateHolder::new();

    let store = TestStore::new();
    let material = StaticChangeResetTokenDestinationMaterial {
        authorize: StaticAuthorizeInfra::new(operator_user_id()),
        destination_repository: standard_destination_repository(&store),
    };
    let fields = StaticChangeResetTokenDestinationFields::Invalid(
        ValidateChangeResetTokenDestinationFieldsError::InvalidTo(
            ValidateResetPasswordTokenDestinationError::Email(ValidateTextError::TooLong),
        ),
    );

    let mut action = ChangeResetTokenDestinationAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(StaticAuthorizeToken, fields).await;
    assert_eq!(
        holder.extract(),
        vec![
            "try to proxy call: auth.ticket.authorize.clarify(require: some [auth-user])",
            "proxy call success",
            "change reset token destination error; invalid; to: reset-token-destination: email: too long",
        ],
    );
    assert!(result.is_err());
}

struct TestStore {
    destination: MemoryAuthUserStore,
}

impl TestStore {
    fn new() -> Self {
        Self {
            destination: MemoryAuthUserStore::new(),
        }
    }
}

fn standard_destination_repository<'a>(store: &'a TestStore) -> MemoryAuthUserRepository<'a> {
    MemoryAuthUserRepository::with_user_id_and_destination(
        &store.destination,
        stored_login_id(),
        stored_user_id(),
        stored_destination(),
    )
}

fn no_destination_repository<'a>(store: &'a TestStore) -> MemoryAuthUserRepository<'a> {
    MemoryAuthUserRepository::with_user_id(&store.destination, stored_login_id(), stored_user_id())
}

fn stored_login_id() -> LoginId {
    LoginId::restore("login-id".into())
}
fn stored_user_id() -> AuthUserId {
    AuthUserId::restore("user-id".into())
}
fn stored_destination() -> ResetPasswordTokenDestination {
    ResetPasswordTokenDestination::Email(ResetPasswordTokenDestinationEmail::restore(
        "destination@example.com".to_owned(),
    ))
}

fn operator_user_id() -> AuthUserId {
    AuthUserId::restore("operator-user-id".to_owned())
}
