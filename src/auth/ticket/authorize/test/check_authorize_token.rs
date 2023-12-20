use pretty_assertions::assert_eq;

use crate::{
    auth::ticket::kernel::detail::token::authorize::decoder::test::MockAuthorizeTokenDecoder,
    common::api::feature::AsInfra,
};

use crate::x_content::permission::AuthPermission;

use crate::auth::ticket::authorize::action::CheckAuthorizeTokenAction;

use crate::auth::{
    ticket::{
        authorize::data::{CheckAuthorizeTokenError, CheckAuthorizeTokenSuccess},
        kernel::data::{
            AuthPermissionError, AuthPermissionGranted, AuthPermissionRequired, AuthTicket,
            AuthTicketAttrs, AuthTicketId, AuthorizeToken, DecodeAuthorizeTokenError,
        },
    },
    user::kernel::data::AuthUserId,
};

#[tokio::test]
async fn success() -> Result<(), CheckAuthorizeTokenError> {
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
    });
    let action = CheckAuthorizeTokenAction::mock(feature.as_infra());

    let token = AuthorizeToken::restore("TOKEN".to_owned());
    let required =
        AuthPermissionRequired::HasSome(vec![AuthPermission::AuthUser].into_iter().collect());

    let auth = action.check(token, required).await?;

    assert_eq!(
        auth,
        CheckAuthorizeTokenSuccess::new(AuthPermissionGranted::restore(
            vec![AuthPermission::AuthUser].into_iter().collect(),
        ))
    );

    Ok(())
}

#[tokio::test]
async fn error_invalid_token() {
    let feature = feature(Infra { decoder: vec![] });
    let action = CheckAuthorizeTokenAction::mock(feature.as_infra());

    let token = AuthorizeToken::restore("TOKEN".to_owned());
    let required =
        AuthPermissionRequired::HasSome(vec![AuthPermission::AuthUser].into_iter().collect());

    let err = action.check(token, required).await.unwrap_err();

    assert_eq!(
        format!("{}", err),
        format!(
            "{}",
            CheckAuthorizeTokenError::DecodeError(DecodeAuthorizeTokenError::Invalid(format!(
                "invalid authorize token"
            )))
        ),
    )
}

#[tokio::test]
async fn error_token_expired() {
    let feature = feature(Infra {
        decoder: vec![("TOKEN".to_owned(), Err(DecodeAuthorizeTokenError::Expired))],
    });
    let action = CheckAuthorizeTokenAction::mock(feature.as_infra());

    let token = AuthorizeToken::restore("TOKEN".to_owned());
    let required =
        AuthPermissionRequired::HasSome(vec![AuthPermission::AuthUser].into_iter().collect());

    let err = action.check(token, required).await.unwrap_err();

    assert_eq!(
        format!("{}", err),
        format!(
            "{}",
            CheckAuthorizeTokenError::DecodeError(DecodeAuthorizeTokenError::Expired)
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
    });
    let action = CheckAuthorizeTokenAction::mock(feature.as_infra());

    let token = AuthorizeToken::restore("TOKEN".to_owned());
    let required =
        AuthPermissionRequired::HasSome(vec![AuthPermission::AuthUser].into_iter().collect());

    let err = action.check(token, required).await.unwrap_err();

    assert_eq!(
        format!("{}", err),
        format!(
            "{}",
            CheckAuthorizeTokenError::PermissionError(AuthPermissionError::PermissionDenied(
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
}

fn feature(infra: Infra) -> MockAuthorizeTokenDecoder {
    MockAuthorizeTokenDecoder::new(infra.decoder)
}
