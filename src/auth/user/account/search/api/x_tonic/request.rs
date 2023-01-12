use crate::auth::user::account::search::y_protobuf::service::SearchAuthUserAccountRequestPb;

use crate::x_content::permission::AuthPermission;

use crate::auth::user::account::search::infra::SearchAuthUserAccountFilterExtract;

use crate::auth::user::{
    account::search::data::{SearchAuthUserAccountFilter, SearchAuthUserAccountFilterProps},
    login_id::kernel::data::SearchLoginId,
};

impl SearchAuthUserAccountFilterExtract for SearchAuthUserAccountRequestPb {
    fn convert(mut self) -> SearchAuthUserAccountFilter {
        SearchAuthUserAccountFilter {
            offset: self.offset.into(),
            sort: self.sort.into(),
            props: SearchAuthUserAccountFilterProps {
                login_id: SearchLoginId::restore(self.login_id.pop()),
                granted: self
                    .granted
                    .into_iter()
                    .filter_map(AuthPermission::convert)
                    .collect(),
            },
        }
    }
}
