use pretty_assertions::assert_eq;

use crate::{
    auth::ticket::kernel::detail::token::authenticate::decoder::test::MockAuthenticateTokenDecoder,
    common::api::feature::AsInfra, x_content::permission::AuthPermission,
};

use crate::auth::ticket::authenticate::action::CheckAuthenticateTokenAction;

use crate::auth::{
    ticket::{
        authenticate::data::{CheckAuthenticateTokenError, CheckAuthenticateTokenSuccess},
        kernel::data::{
            AuthPermissionGranted, AuthTicket, AuthTicketAttrs, AuthTicketId, AuthenticateToken,
            DecodeAuthenticateTokenError,
        },
    },
    user::kernel::data::AuthUserId,
};

#[test]
fn success() -> Result<(), CheckAuthenticateTokenError> {
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
    let action = CheckAuthenticateTokenAction::mock(feature.as_infra());

    let token = AuthenticateToken::restore("TOKEN".to_owned());

    let auth = action.check(token)?;

    assert_eq!(auth, CheckAuthenticateTokenSuccess::new(stored_ticket));

    Ok(())
}

#[test]
fn error_invalid_token() {
    let feature = feature(Infra { decoder: vec![] });
    let action = CheckAuthenticateTokenAction::mock(feature.as_infra());

    let token = AuthenticateToken::restore("TOKEN".to_owned());

    let err = action.check(token).unwrap_err();

    assert_eq!(
        format!("{}", err),
        format!(
            "{}",
            CheckAuthenticateTokenError::DecodeError(DecodeAuthenticateTokenError::Invalid(
                format!("invalid authenticate token")
            ))
        ),
    )
}

struct Infra {
    decoder: Vec<(String, Result<AuthTicket, DecodeAuthenticateTokenError>)>,
}

fn feature(infra: Infra) -> MockAuthenticateTokenDecoder {
    MockAuthenticateTokenDecoder::new(infra.decoder)
}
