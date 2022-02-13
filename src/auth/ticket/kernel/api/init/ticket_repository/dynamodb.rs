use std::collections::HashMap;

use chrono::{DateTime, NaiveDateTime, Utc};
use rusoto_dynamodb::{
    AttributeValue, DeleteItemInput, DynamoDb, DynamoDbClient, GetItemInput, PutItemInput,
};

use crate::auth::x_outside_feature::feature::AuthOutsideStore;

use crate::z_lib::repository::{
    dynamodb::helper::{string_value, timestamp_value},
    helper::infra_error,
};

use crate::auth::ticket::{
    encode::infra::EncodeAuthTicketRepository, issue::infra::IssueAuthTicketRepository,
    logout::infra::LogoutAuthTicketRepository,
};

use crate::{
    auth::ticket::kernel::data::{AuthDateTime, AuthTicket, ExpansionLimitDateTime},
    z_lib::repository::data::RepositoryError,
};

pub struct DynamoDbAuthTicketRepository<'a> {
    client: &'a DynamoDbClient,
    ticket: &'a str,
}

impl<'a> DynamoDbAuthTicketRepository<'a> {
    pub const fn new(feature: &'a AuthOutsideStore) -> Self {
        Self {
            client: &feature.dynamodb,
            ticket: feature.ticket_table_name,
        }
    }
}

#[async_trait::async_trait]
impl<'a> IssueAuthTicketRepository for DynamoDbAuthTicketRepository<'a> {
    async fn issue(
        &self,
        ticket: AuthTicket,
        expansion_limit: ExpansionLimitDateTime,
        issued_at: AuthDateTime,
    ) -> Result<(), RepositoryError> {
        issue(self, ticket, expansion_limit, issued_at).await
    }
}
async fn issue<'a>(
    repository: &DynamoDbAuthTicketRepository<'a>,
    ticket: AuthTicket,
    expansion_limit: ExpansionLimitDateTime,
    issued_at: AuthDateTime,
) -> Result<(), RepositoryError> {
    let mut item = AttributeMap::new();
    item.add_ticket(ticket);
    item.add_expansion_limit(expansion_limit);
    item.add_issued_at(issued_at);

    // 有効期限が切れた項目は dynamodb の TTL の設定によって削除される
    let input = PutItemInput {
        table_name: repository.ticket.into(),
        condition_expression: Some("attribute_not_exists(ticket_id)".into()),
        item: item.extract(),
        ..Default::default()
    };

    repository
        .client
        .put_item(input)
        .await
        .map_err(infra_error)?;

    Ok(())
}

#[async_trait::async_trait]
impl<'a> LogoutAuthTicketRepository for DynamoDbAuthTicketRepository<'a> {
    async fn discard(&self, ticket: AuthTicket) -> Result<(), RepositoryError> {
        discard(self, ticket).await
    }
}
async fn discard<'a>(
    repository: &DynamoDbAuthTicketRepository<'a>,
    ticket: AuthTicket,
) -> Result<(), RepositoryError> {
    let mut item = AttributeMap::new();
    item.add_ticket(ticket);

    let input = DeleteItemInput {
        table_name: repository.ticket.into(),
        key: item.extract(),
        ..Default::default()
    };

    repository
        .client
        .delete_item(input)
        .await
        .map_err(infra_error)?;

    Ok(())
}

#[async_trait::async_trait]
impl<'a> EncodeAuthTicketRepository for DynamoDbAuthTicketRepository<'a> {
    async fn find_expansion_limit(
        &self,
        ticket: &AuthTicket,
    ) -> Result<Option<ExpansionLimitDateTime>, RepositoryError> {
        find_expansion_limit(self, ticket).await
    }
}
async fn find_expansion_limit<'a>(
    repository: &DynamoDbAuthTicketRepository<'a>,
    ticket: &AuthTicket,
) -> Result<Option<ExpansionLimitDateTime>, RepositoryError> {
    let mut item = AttributeMap::new();
    item.add_ticket(ticket.clone());

    let input = GetItemInput {
        table_name: repository.ticket.into(),
        key: item.extract(),
        projection_expression: Some("expansion_limit".into()),
        ..Default::default()
    };

    let response = repository
        .client
        .get_item(input)
        .await
        .map_err(infra_error)?;

    let found = response
        .item
        .and_then(|mut attrs| attrs.remove("expansion_limit"))
        .and_then(|attr| attr.n)
        .and_then(|found| found.parse::<i64>().ok());

    Ok(found.map(|expansion_limit| {
        ExpansionLimitDateTime::restore(DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp(expansion_limit, 0),
            Utc,
        ))
    }))
}

struct AttributeMap(HashMap<String, AttributeValue>);

impl AttributeMap {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn extract(self) -> HashMap<String, AttributeValue> {
        self.0
    }

    fn add(&mut self, key: &str, attr: AttributeValue) {
        self.0.insert(key.into(), attr);
    }

    fn add_ticket(&mut self, ticket: AuthTicket) {
        let ticket = ticket.extract();

        self.add("ticket_id", string_value(ticket.ticket_id));
        self.add("user_id", string_value(ticket.user_id));
    }
    fn add_expansion_limit(&mut self, expansion_limit: ExpansionLimitDateTime) {
        self.add(
            "expansion_limit",
            timestamp_value(expansion_limit.extract()),
        );
    }
    fn add_issued_at(&mut self, issued_at: AuthDateTime) {
        self.add("issued_at", timestamp_value(issued_at.extract()));
    }
}
