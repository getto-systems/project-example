use getto_application::data::MethodResult;

use crate::auth::ticket::remote::{
    discard::infra::DiscardAuthTicketInfra,
    kernel::infra::{AuthClock, DiscardAuthTicketRepository},
};

use crate::auth::ticket::remote::validate::method::validate_auth_token;

use super::event::DiscardAuthTicketEvent;

use crate::auth::user::remote::kernel::data::RequireAuthRoles;

pub async fn discard_auth_ticket<S>(
    infra: &impl DiscardAuthTicketInfra,
    post: impl Fn(DiscardAuthTicketEvent) -> S,
) -> MethodResult<S> {
    let ticket = validate_auth_token(infra.validate_infra(), RequireAuthRoles::Nothing, |event| {
        post(DiscardAuthTicketEvent::Validate(event))
    })
    .await?;

    let clock = infra.clock();
    let ticket_repository = infra.ticket_repository();

    ticket_repository
        .discard(ticket, clock.now())
        .await
        .map_err(|err| post(DiscardAuthTicketEvent::RepositoryError(err)))?;

    Ok(post(DiscardAuthTicketEvent::Success))
}
