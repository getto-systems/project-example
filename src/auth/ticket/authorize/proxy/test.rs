use pretty_assertions::assert_eq;

use crate::auth::ticket::kernel::detail::token::authorize::decoder::test::MockAuthorizeTokenDecoder;

use crate::x_content::permission::AuthPermission;

use crate::common::api::feature::AsInfra;

use crate::auth::ticket::authorize::proxy::action::AuthorizeProxyAction;

use crate::{
    auth::{
        ticket::{
            authorize::{data::AuthorizeSuccess, proxy::data::AuthorizeProxyError},
            kernel::data::{
                AuthPermissionError, AuthPermissionGranted, AuthPermissionRequired, AuthTicket,
                AuthTicketAttrs, AuthTicketId, AuthorizeToken, DecodeAuthorizeTokenError,
            },
        },
        user::kernel::data::AuthUserId,
    },
    common::api::request::data::RequestInfo,
};

#[tokio::test]
async fn success() -> Result<(), AuthorizeProxyError> {
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
        decoder: vec![("TOKEN".to_owned(), Ok(stored_ticket.clone()))],
        attrs: stored_ticket.attrs.clone(),
    });
    let action = AuthorizeProxyAction::mock(feature.as_infra());

    let info = RequestInfo::default();
    let token = AuthorizeToken::restore("TOKEN".to_owned());
    let required =
        AuthPermissionRequired::HasSome(vec![AuthPermission::AuthUser].into_iter().collect());

    let auth = action.authorize(info, token, required).await?;

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
        decoder: vec![],
        attrs: stored_ticket.attrs,
    });
    let action = AuthorizeProxyAction::mock(feature.as_infra());

    let info = RequestInfo::default();
    let token = AuthorizeToken::restore("TOKEN".to_owned());
    let required =
        AuthPermissionRequired::HasSome(vec![AuthPermission::AuthUser].into_iter().collect());

    let err = action.authorize(info, token, required).await.unwrap_err();

    assert_eq!(
        format!("{}", err),
        format!(
            "{}",
            AuthorizeProxyError::DecodeError(DecodeAuthorizeTokenError::Invalid(format!(
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
        decoder: vec![("TOKEN".to_owned(), Err(DecodeAuthorizeTokenError::Expired))],
        attrs: stored_ticket.attrs,
    });
    let action = AuthorizeProxyAction::mock(feature.as_infra());

    let info = RequestInfo::default();
    let token = AuthorizeToken::restore("TOKEN".to_owned());
    let required =
        AuthPermissionRequired::HasSome(vec![AuthPermission::AuthUser].into_iter().collect());

    let err = action.authorize(info, token, required).await.unwrap_err();

    assert_eq!(
        format!("{}", err),
        format!(
            "{}",
            AuthorizeProxyError::DecodeError(DecodeAuthorizeTokenError::Expired)
        ),
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
        decoder: vec![("TOKEN".to_owned(), Ok(stored_ticket.clone()))],
        attrs: stored_ticket.attrs,
    });
    let action = AuthorizeProxyAction::mock(feature.as_infra());

    let info = RequestInfo::default();
    let token = AuthorizeToken::restore("TOKEN".to_owned());
    let required =
        AuthPermissionRequired::HasSome(vec![AuthPermission::AuthUser].into_iter().collect());

    let err = action.authorize(info, token, required).await.unwrap_err();

    assert_eq!(
        format!("{}", err),
        format!(
            "{}",
            AuthorizeProxyError::PermissionError(AuthPermissionError::PermissionDenied(
                AuthPermissionGranted::restore(vec![].into_iter().collect()),
                AuthPermissionRequired::HasSome(
                    vec![AuthPermission::AuthUser].into_iter().collect(),
                )
            ))
        ),
    )
}

struct Infra {
    decoder: Vec<(String, Result<AuthTicket, DecodeAuthorizeTokenError>)>,
    attrs: AuthTicketAttrs,
}

fn feature(infra: Infra) -> (MockAuthorizeTokenDecoder, AuthorizeSuccess) {
    (
        MockAuthorizeTokenDecoder::new(infra.decoder),
        AuthorizeSuccess::new(infra.attrs),
    )
}
