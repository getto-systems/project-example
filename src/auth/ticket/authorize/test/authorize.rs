use std::sync::Arc;

use pretty_assertions::assert_eq;

use chrono::{DateTime, TimeZone, Utc};

use crate::{
    auth::{
        kernel::detail::test::MockChronoAuthClock,
        ticket::kernel::detail::{
            repository::memory::{ticket::MapTicket, StoreTicket},
            token::authorize::decoder::test::MockAuthorizeTokenDecoder,
        },
        user::kernel::detail::repository::memory::{user::MapUser, StoreUser},
    },
    common::api::feature::AsInfra,
};

use crate::x_content::permission::AuthPermission;

use crate::auth::ticket::authorize::action::AuthorizeAction;

use crate::auth::ticket::authorize::infra::AuthorizeFields;

use crate::auth::{
    kernel::data::{AuthDateTime, ExpansionLimitDateTime},
    ticket::{
        authorize::data::{AuthorizeError, AuthorizeSuccess},
        kernel::data::{
            AuthPermissionError, AuthPermissionGranted, AuthPermissionRequired, AuthTicket,
            AuthTicketAttrs, AuthTicketId, AuthorizeToken, DecodeAuthorizeTokenError,
        },
    },
    user::{kernel::data::AuthUserId, login_id::kernel::data::LoginId},
};

#[tokio::test]
async fn success() -> Result<(), AuthorizeError> {
    let stored_ticket = AuthTicket {
        ticket_id: AuthTicketId::restore("ticket-id".to_owned()),
        attrs: AuthTicketAttrs {
            user_id: AuthUserId::restore("user-id".to_owned()),
            granted: AuthPermissionGranted::restore(
                vec![AuthPermission::AuthUser].into_iter().collect(),
            ),
        },
    };

    let feature = feature(Infra {
        now: Utc.with_ymd_and_hms(2021, 1, 1, 10, 0, 0).unwrap(),
        decoder: vec![("TOKEN".to_owned(), Ok(stored_ticket.clone()))],
        ticket: vec![(
            stored_ticket.clone(),
            Utc.with_ymd_and_hms(2021, 1, 2, 10, 0, 0).unwrap(),
        )],
    });
    let action = AuthorizeAction::mock(feature.as_infra());

    let fields = AuthorizeFields {
        token: AuthorizeToken::restore("TOKEN".to_owned()),
        required: AuthPermissionRequired::HasSome(
            vec![AuthPermission::AuthUser].into_iter().collect(),
        ),
    };

    let auth = action.authorize(fields).await?;

    assert_eq!(auth, AuthorizeSuccess::new(stored_ticket.attrs));

    Ok(())
}

#[tokio::test]
async fn error_invalid_token() {
    let stored_ticket = AuthTicket {
        ticket_id: AuthTicketId::restore("ticket-id".to_owned()),
        attrs: AuthTicketAttrs {
            user_id: AuthUserId::restore("user-id".to_owned()),
            granted: AuthPermissionGranted::restore(
                vec![AuthPermission::AuthUser].into_iter().collect(),
            ),
        },
    };

    let feature = feature(Infra {
        now: Utc.with_ymd_and_hms(2021, 1, 1, 10, 0, 0).unwrap(),
        decoder: vec![],
        ticket: vec![(
            stored_ticket.clone(),
            Utc.with_ymd_and_hms(2021, 1, 2, 10, 0, 0).unwrap(),
        )],
    });
    let action = AuthorizeAction::mock(feature.as_infra());

    let fields = AuthorizeFields {
        token: AuthorizeToken::restore("TOKEN".to_owned()),
        required: AuthPermissionRequired::HasSome(
            vec![AuthPermission::AuthUser].into_iter().collect(),
        ),
    };

    let err = action.authorize(fields).await.unwrap_err();

    assert_eq!(
        format!("{}", err),
        format!(
            "{}",
            AuthorizeError::DecodeError(DecodeAuthorizeTokenError::Invalid(format!(
                "invalid authorize token"
            )))
        ),
    )
}

#[tokio::test]
async fn error_token_expired() {
    let stored_ticket = AuthTicket {
        ticket_id: AuthTicketId::restore("ticket-id".to_owned()),
        attrs: AuthTicketAttrs {
            user_id: AuthUserId::restore("user-id".to_owned()),
            granted: AuthPermissionGranted::restore(
                vec![AuthPermission::AuthUser].into_iter().collect(),
            ),
        },
    };

    let feature = feature(Infra {
        now: Utc.with_ymd_and_hms(2021, 1, 1, 10, 0, 0).unwrap(),
        decoder: vec![("TOKEN".to_owned(), Err(DecodeAuthorizeTokenError::Expired))],
        ticket: vec![(
            stored_ticket.clone(),
            Utc.with_ymd_and_hms(2021, 1, 2, 10, 0, 0).unwrap(),
        )],
    });
    let action = AuthorizeAction::mock(feature.as_infra());

    let fields = AuthorizeFields {
        token: AuthorizeToken::restore("TOKEN".to_owned()),
        required: AuthPermissionRequired::HasSome(
            vec![AuthPermission::AuthUser].into_iter().collect(),
        ),
    };

    let err = action.authorize(fields).await.unwrap_err();

    assert_eq!(
        format!("{}", err),
        format!(
            "{}",
            AuthorizeError::DecodeError(DecodeAuthorizeTokenError::Expired)
        ),
    )
}

#[tokio::test]
async fn error_no_ticket() {
    let stored_ticket = AuthTicket {
        ticket_id: AuthTicketId::restore("ticket-id".to_owned()),
        attrs: AuthTicketAttrs {
            user_id: AuthUserId::restore("user-id".to_owned()),
            granted: AuthPermissionGranted::restore(
                vec![AuthPermission::AuthUser].into_iter().collect(),
            ),
        },
    };

    let feature = feature(Infra {
        now: Utc.with_ymd_and_hms(2021, 1, 1, 10, 0, 0).unwrap(),
        decoder: vec![("TOKEN".to_owned(), Ok(stored_ticket.clone()))],
        ticket: vec![],
    });
    let action = AuthorizeAction::mock(feature.as_infra());

    let fields = AuthorizeFields {
        token: AuthorizeToken::restore("TOKEN".to_owned()),
        required: AuthPermissionRequired::HasSome(
            vec![AuthPermission::AuthUser].into_iter().collect(),
        ),
    };

    let err = action.authorize(fields).await.unwrap_err();

    assert_eq!(
        format!("{}", err),
        format!("{}", AuthorizeError::TicketNotFound),
    )
}

#[tokio::test]
async fn error_expired_ticket() {
    let stored_ticket = AuthTicket {
        ticket_id: AuthTicketId::restore("ticket-id".to_owned()),
        attrs: AuthTicketAttrs {
            user_id: AuthUserId::restore("user-id".to_owned()),
            granted: AuthPermissionGranted::restore(
                vec![AuthPermission::AuthUser].into_iter().collect(),
            ),
        },
    };

    let feature = feature(Infra {
        now: Utc.with_ymd_and_hms(2021, 1, 1, 10, 0, 0).unwrap(),
        decoder: vec![("TOKEN".to_owned(), Ok(stored_ticket.clone()))],
        ticket: vec![(
            stored_ticket.clone(),
            Utc.with_ymd_and_hms(2021, 1, 1, 9, 0, 0).unwrap(),
        )],
    });
    let action = AuthorizeAction::mock(feature.as_infra());

    let fields = AuthorizeFields {
        token: AuthorizeToken::restore("TOKEN".to_owned()),
        required: AuthPermissionRequired::HasSome(
            vec![AuthPermission::AuthUser].into_iter().collect(),
        ),
    };

    let err = action.authorize(fields).await.unwrap_err();

    assert_eq!(
        format!("{}", err),
        format!("{}", AuthorizeError::TicketHasExpired),
    )
}

#[tokio::test]
async fn error_permission_denied() {
    let stored_ticket = AuthTicket {
        ticket_id: AuthTicketId::restore("ticket-id".to_owned()),
        attrs: AuthTicketAttrs {
            user_id: AuthUserId::restore("user-id".to_owned()),
            granted: AuthPermissionGranted::restore(vec![].into_iter().collect()),
        },
    };

    let feature = feature(Infra {
        now: Utc.with_ymd_and_hms(2021, 1, 1, 10, 0, 0).unwrap(),
        decoder: vec![("TOKEN".to_owned(), Ok(stored_ticket.clone()))],
        ticket: vec![(
            stored_ticket.clone(),
            Utc.with_ymd_and_hms(2021, 1, 2, 10, 0, 0).unwrap(),
        )],
    });
    let action = AuthorizeAction::mock(feature.as_infra());

    let fields = AuthorizeFields {
        token: AuthorizeToken::restore("TOKEN".to_owned()),
        required: AuthPermissionRequired::HasSome(
            vec![AuthPermission::AuthUser].into_iter().collect(),
        ),
    };

    let err = action.authorize(fields).await.unwrap_err();

    assert_eq!(
        format!("{}", err),
        format!(
            "{}",
            AuthorizeError::PermissionError(AuthPermissionError::PermissionDenied(
                AuthPermissionGranted::restore(vec![].into_iter().collect()),
                AuthPermissionRequired::HasSome(
                    vec![AuthPermission::AuthUser].into_iter().collect(),
                )
            ))
        ),
    )
}

struct Infra {
    now: DateTime<Utc>,
    decoder: Vec<(String, Result<AuthTicket, DecodeAuthorizeTokenError>)>,
    ticket: Vec<(AuthTicket, DateTime<Utc>)>,
}

fn feature(
    infra: Infra,
) -> (
    MockAuthorizeTokenDecoder,
    Arc<StoreTicket>,
    Arc<StoreUser>,
    MockChronoAuthClock,
) {
    let token_decoder = MockAuthorizeTokenDecoder::new(infra.decoder);

    let ticket_store = Arc::new(StoreTicket::default());
    let user_store = Arc::new(StoreUser::default());

    for (ticket, expansion_limit) in infra.ticket {
        let login_id = LoginId::restore(format!("login-id({})", ticket.attrs.user_id));

        MapTicket::insert_entry(
            &ticket_store,
            ticket.ticket_id,
            (
                ticket.attrs.user_id.clone(),
                ExpansionLimitDateTime::restore(expansion_limit),
                AuthDateTime::restore(infra.now.clone()),
            ),
        );
        MapUser::insert_entry(
            &user_store,
            ticket.attrs.user_id,
            (login_id, Some(ticket.attrs.granted), None, None),
        )
    }

    (
        token_decoder,
        ticket_store,
        user_store,
        MockChronoAuthClock::new(infra.now),
    )
}
