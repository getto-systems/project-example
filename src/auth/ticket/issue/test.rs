use std::sync::Arc;

use chrono::{DateTime, Duration, TimeZone, Utc};

use crate::auth::{
    kernel::detail::test::MockChronoAuthClock,
    ticket::{
        issue::detail::test::MockAuthTicketIdGenerator,
        kernel::detail::repository::memory::StoreTicket,
    },
};

use crate::{common::api::feature::AsInfra, x_content::permission::AuthPermission};

use crate::auth::ticket::issue::action::IssueAuthTicketAction;

use crate::auth::ticket::issue::infra::IssueAuthTicketConfig;

use crate::auth::{
    data::ExpansionLimitDuration,
    ticket::{
        issue::data::IssueAuthTicketError,
        kernel::data::{
            AuthPermissionGranted, AuthTicket, AuthTicketAttrs, AuthTicketId, AuthenticateSuccess,
        },
    },
    user::kernel::data::{AuthUser, AuthUserId},
};

#[tokio::test]
async fn success() -> Result<(), IssueAuthTicketError> {
    let feature = feature(Infra {
        now: Utc.with_ymd_and_hms(2021, 1, 1, 10, 0, 0).unwrap(),
        ticket_id: AuthTicketId::restore("ticket-id".to_owned()),
        config: IssueAuthTicketConfig {
            authenticate_expansion_limit: ExpansionLimitDuration::with_duration(Duration::days(1)),
        },
    });
    let action = IssueAuthTicketAction::mock(feature.as_infra());

    let auth = AuthenticateSuccess::new(AuthUser {
        user_id: AuthUserId::restore("user-id".to_owned()),
        granted: AuthPermissionGranted::restore(
            vec![AuthPermission::AuthUser].into_iter().collect(),
        ),
    });

    let ticket = action.issue(auth).await?;

    assert_eq!(
        ticket,
        AuthTicket {
            ticket_id: AuthTicketId::restore("ticket-id".to_owned()),
            attrs: AuthTicketAttrs {
                user_id: AuthUserId::restore("user-id".to_owned()),
                granted: AuthPermissionGranted::restore(
                    vec![AuthPermission::AuthUser].into_iter().collect(),
                ),
            },
        },
    );

    Ok(())
}

struct Infra {
    now: DateTime<Utc>,
    ticket_id: AuthTicketId,
    config: IssueAuthTicketConfig,
}

fn feature(
    infra: Infra,
) -> (
    MockChronoAuthClock,
    Arc<StoreTicket>,
    MockAuthTicketIdGenerator,
    IssueAuthTicketConfig,
) {
    (
        MockChronoAuthClock::new(infra.now),
        Arc::new(StoreTicket::default()),
        MockAuthTicketIdGenerator::new(infra.ticket_id),
        infra.config,
    )
}
