use std::sync::Arc;

use pretty_assertions::assert_eq;

use chrono::{DateTime, TimeZone, Utc};

use crate::{
    auth::ticket::kernel::detail::repository::memory::{ticket::MapTicket, StoreTicket},
    common::api::feature::AsInfra,
    x_content::permission::AuthPermission,
};

use crate::auth::ticket::logout::action::LogoutAction;

use crate::auth::{
    kernel::data::{AuthDateTime, ExpansionLimitDateTime},
    ticket::{
        authenticate::data::CheckAuthenticateTokenSuccess,
        kernel::data::{AuthPermissionGranted, AuthTicket, AuthTicketAttrs, AuthTicketId},
        logout::data::{LogoutError, LogoutSuccess},
    },
    user::kernel::data::AuthUserId,
};

#[tokio::test]
async fn success() -> Result<(), LogoutError> {
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
        ticket: vec![(
            stored_ticket.clone(),
            Utc.with_ymd_and_hms(2021, 1, 2, 10, 0, 0).unwrap(),
        )],
    });
    let action = LogoutAction::mock(feature.as_infra());

    let auth = CheckAuthenticateTokenSuccess::new(stored_ticket.clone());

    let auth = action.logout(auth).await?;

    assert_eq!(auth, LogoutSuccess::new(stored_ticket));

    Ok(())
}

struct Infra {
    now: DateTime<Utc>,
    ticket: Vec<(AuthTicket, DateTime<Utc>)>,
}

fn feature(infra: Infra) -> Arc<StoreTicket> {
    let ticket_store = Arc::new(StoreTicket::default());

    for (ticket, expansion_limit) in infra.ticket {
        MapTicket::insert_entry(
            &ticket_store,
            ticket.ticket_id,
            (
                ticket.attrs.user_id.clone(),
                ExpansionLimitDateTime::restore(expansion_limit),
                AuthDateTime::restore(infra.now.clone()),
            ),
        );
    }

    ticket_store
}
