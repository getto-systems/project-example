use crate::auth::user::account::remote::y_protobuf::service::SearchUserAccountRequestPb;

use crate::auth::user::account::remote::search::infra::{
    SearchUserAccountFieldsExtract, SearchUserAccountRequestDecoder,
};
use crate::z_lib::remote::search::data::SearchSortExtract;

pub struct PbSearchUserAccountRequestDecoder {
    request: SearchUserAccountRequestPb,
}

impl PbSearchUserAccountRequestDecoder {
    pub const fn new(request: SearchUserAccountRequestPb) -> Self {
        Self { request }
    }
}

impl SearchUserAccountRequestDecoder for PbSearchUserAccountRequestDecoder {
    fn decode(self) -> SearchUserAccountFieldsExtract {
        SearchUserAccountFieldsExtract {
            offset: self.request.offset,
            sort: SearchSortExtract {
                key: self.request.sort_key,
                order: self.request.sort_order,
            },
            login_id: self.request.login_id,
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::user::account::remote::search::infra::{
        SearchUserAccountFieldsExtract, SearchUserAccountRequestDecoder,
    };

    pub enum StaticSearchUserAccountRequestDecoder {
        Valid(SearchUserAccountFieldsExtract),
    }

    impl SearchUserAccountRequestDecoder for StaticSearchUserAccountRequestDecoder {
        fn decode(self) -> SearchUserAccountFieldsExtract {
            match self {
                Self::Valid(fields) => fields,
            }
        }
    }
}
