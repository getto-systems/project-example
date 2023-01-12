use sea_query::Order;

use crate::common::api::search::data::SearchSortDirection;

impl Into<Order> for SearchSortDirection {
    fn into(self) -> Order {
        match self {
            Self::Asc => Order::Asc,
            Self::Desc => Order::Desc,
        }
    }
}
