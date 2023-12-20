use std::sync::Arc;

use pretty_assertions::assert_eq;

use chrono::{DateTime, Duration, TimeZone, Utc};

use crate::{
    auth::{
        kernel::detail::test::MockChronoAuthClock,
        ticket::kernel::detail::repository::memory::{ticket::MapTicket, StoreTicket},
    },
    common::api::feature::AsInfra,
    x_content::permission::AuthPermission,
};

use crate::auth::ticket::encode::action::EncodeAuthTokenAction;

use crate::auth::ticket::encode::infra::EncodeAuthTokenConfig;

use crate::auth::{
    kernel::data::{AuthDateTime, ExpansionLimitDateTime, ExpireDateTime, ExpireDuration},
    ticket::{
        encode::data::{EncodeAuthTokenError, EncodeAuthTokenSuccess},
        kernel::data::{
            AWSCloudfrontToken, AuthPermissionGranted, AuthTicket, AuthTicketAttrs, AuthTicketId,
            AuthToken, AuthenticateToken, AuthorizeToken, CdnToken,
        },
    },
    user::kernel::data::AuthUserId,
};

#[tokio::test]
async fn success() -> Result<(), EncodeAuthTokenError> {
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
            Utc.with_ymd_and_hms(2021, 1, 3, 10, 0, 0).unwrap(),
        )],
        config: EncodeAuthTokenConfig {
            authenticate_expires: ExpireDuration::with_duration(Duration::days(1)),
            authorize_expires: ExpireDuration::with_duration(Duration::minutes(1)),
            cdn_expires: ExpireDuration::with_duration(Duration::minutes(1)),
        },
    });
    let action = EncodeAuthTokenAction::mock(feature.as_infra());

    let auth = action.encode(stored_ticket).await?;

    assert_eq!(
        auth,
        EncodeAuthTokenSuccess::new(
            AuthToken {
                authenticate_token: (
                    AuthenticateToken::restore("TOKEN".to_owned()),
                    ExpireDateTime::restore(Utc.with_ymd_and_hms(2021, 1, 2, 10, 0, 0).unwrap()),
                ),
                authorize_token: (
                    AuthorizeToken::restore("TOKEN".to_owned()),
                    ExpireDateTime::restore(Utc.with_ymd_and_hms(2021, 1, 1, 10, 1, 0).unwrap())
                ),
                cdn_token: (
                    CdnToken::AWSCloudfront(AWSCloudfrontToken {
                        key_pair_id: "KEY-PAIR-ID".to_owned(),
                        policy: "POLICY".to_owned(),
                        signature: "SIGNATURE".to_owned(),
                    }),
                    ExpireDateTime::restore(Utc.with_ymd_and_hms(2021, 1, 1, 10, 1, 0).unwrap()),
                ),
            },
            AuthPermissionGranted::restore(vec![AuthPermission::AuthUser].into_iter().collect()),
        )
    );

    Ok(())
}

#[tokio::test]
async fn success_expansion_limited() -> Result<(), EncodeAuthTokenError> {
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
            Utc.with_ymd_and_hms(2021, 1, 1, 12, 0, 0).unwrap(),
        )],
        config: EncodeAuthTokenConfig {
            authenticate_expires: ExpireDuration::with_duration(Duration::days(1)),
            authorize_expires: ExpireDuration::with_duration(Duration::minutes(1)),
            cdn_expires: ExpireDuration::with_duration(Duration::minutes(1)),
        },
    });
    let action = EncodeAuthTokenAction::mock(feature.as_infra());

    let auth = action.encode(stored_ticket).await?;

    assert_eq!(
        auth,
        EncodeAuthTokenSuccess::new(
            AuthToken {
                authenticate_token: (
                    AuthenticateToken::restore("TOKEN".to_owned()),
                    ExpireDateTime::restore(Utc.with_ymd_and_hms(2021, 1, 1, 12, 0, 0).unwrap()),
                ),
                authorize_token: (
                    AuthorizeToken::restore("TOKEN".to_owned()),
                    ExpireDateTime::restore(Utc.with_ymd_and_hms(2021, 1, 1, 10, 1, 0).unwrap())
                ),
                cdn_token: (
                    CdnToken::AWSCloudfront(AWSCloudfrontToken {
                        key_pair_id: "KEY-PAIR-ID".to_owned(),
                        policy: "POLICY".to_owned(),
                        signature: "SIGNATURE".to_owned(),
                    }),
                    ExpireDateTime::restore(Utc.with_ymd_and_hms(2021, 1, 1, 10, 1, 0).unwrap()),
                ),
            },
            AuthPermissionGranted::restore(vec![AuthPermission::AuthUser].into_iter().collect()),
        )
    );

    Ok(())
}

#[tokio::test]
async fn error_no_ticket() {
    let ticket = AuthTicket {
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
        ticket: vec![],
        config: EncodeAuthTokenConfig {
            authenticate_expires: ExpireDuration::with_duration(Duration::days(1)),
            authorize_expires: ExpireDuration::with_duration(Duration::minutes(1)),
            cdn_expires: ExpireDuration::with_duration(Duration::minutes(1)),
        },
    });
    let action = EncodeAuthTokenAction::mock(feature.as_infra());

    let err = action.encode(ticket).await.unwrap_err();

    assert_eq!(
        format!("{}", err),
        format!("{}", EncodeAuthTokenError::TicketNotFound),
    );
}

struct Infra {
    now: DateTime<Utc>,
    ticket: Vec<(AuthTicket, DateTime<Utc>)>,
    config: EncodeAuthTokenConfig,
}

fn feature(infra: Infra) -> (MockChronoAuthClock, Arc<StoreTicket>, EncodeAuthTokenConfig) {
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

    (
        MockChronoAuthClock::new(infra.now),
        ticket_store,
        infra.config,
    )
}
