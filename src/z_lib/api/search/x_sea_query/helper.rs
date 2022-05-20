use sea_query::Order;

use crate::z_lib::search::data::SearchSortOrderExtract;

impl Into<Order> for SearchSortOrderExtract {
    fn into(self) -> Order {
        match self {
            Self::Asc => Order::Asc,
            Self::Desc => Order::Desc,
        }
    }
}
