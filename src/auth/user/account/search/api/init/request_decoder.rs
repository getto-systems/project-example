use crate::auth::user::account::search::y_protobuf::service::SearchAuthUserAccountRequestPb;

use crate::auth::user::account::search::infra::SearchAuthUserAccountRequestDecoder;

use crate::{
    auth::user::account::search::data::SearchAuthUserAccountFilterExtract,
    z_lib::search::data::SearchSortExtract,
};

pub struct PbSearchAuthUserAccountRequestDecoder {
    request: SearchAuthUserAccountRequestPb,
}

impl PbSearchAuthUserAccountRequestDecoder {
    pub const fn new(request: SearchAuthUserAccountRequestPb) -> Self {
        Self { request }
    }
}

impl SearchAuthUserAccountRequestDecoder for PbSearchAuthUserAccountRequestDecoder {
    fn decode(mut self) -> SearchAuthUserAccountFilterExtract {
        SearchAuthUserAccountFilterExtract {
            offset: self.request.offset,
            sort: SearchSortExtract {
                key: self.request.sort_key.into(),
                order: self.request.sort_order,
            },
            login_id: self.request.login_id.pop(),
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::user::account::search::infra::SearchAuthUserAccountRequestDecoder;

    use crate::auth::user::account::search::data::SearchAuthUserAccountFilterExtract;

    pub enum StaticSearchAuthUserAccountRequestDecoder {
        Valid(SearchAuthUserAccountFilterExtract),
    }

    impl SearchAuthUserAccountRequestDecoder for StaticSearchAuthUserAccountRequestDecoder {
        fn decode(self) -> SearchAuthUserAccountFilterExtract {
            match self {
                Self::Valid(fields) => fields,
            }
        }
    }
}
