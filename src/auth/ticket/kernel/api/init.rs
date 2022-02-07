pub mod clock;
pub mod response_builder;
pub mod ticket_repository;

use crate::auth::x_outside_feature::auth::feature::AuthOutsideStore;

use crate::auth::ticket::kernel::init::ticket_repository::dynamodb::DynamoDbAuthTicketRepository;

pub fn new_auth_ticket_repository<'a>(
    feature: &'a AuthOutsideStore,
) -> DynamoDbAuthTicketRepository<'a> {
    DynamoDbAuthTicketRepository::new(&feature.dynamodb, &feature.ticket_table_name)
}
