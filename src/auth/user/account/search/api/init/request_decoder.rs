use crate::auth::user::account::search::y_protobuf::service::SearchAuthUserAccountRequestPb;

use crate::auth::user::account::search::infra::{
    SearchAuthUserAccountFieldsExtract, SearchAuthUserAccountRequestDecoder,
};
use crate::z_lib::search::data::SearchSortExtract;

pub struct PbSearchAuthUserAccountRequestDecoder {
    request: SearchAuthUserAccountRequestPb,
}

impl PbSearchAuthUserAccountRequestDecoder {
    pub const fn new(request: SearchAuthUserAccountRequestPb) -> Self {
        Self { request }
    }
}

impl SearchAuthUserAccountRequestDecoder for PbSearchAuthUserAccountRequestDecoder {
    fn decode(self) -> SearchAuthUserAccountFieldsExtract {
        SearchAuthUserAccountFieldsExtract {
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
    use crate::auth::user::account::search::infra::{
        SearchAuthUserAccountFieldsExtract, SearchAuthUserAccountRequestDecoder,
    };

    pub enum StaticSearchAuthUserAccountRequestDecoder {
        Valid(SearchAuthUserAccountFieldsExtract),
    }

    impl SearchAuthUserAccountRequestDecoder for StaticSearchAuthUserAccountRequestDecoder {
        fn decode(self) -> SearchAuthUserAccountFieldsExtract {
            match self {
                Self::Valid(fields) => fields,
            }
        }
    }
}
