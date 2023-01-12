use crate::auth::user::account::register::y_protobuf::service::RegisterAuthUserAccountRequestPb;

use crate::auth::user::account::register::infra::RegisterAuthUserAccountFieldsExtract;

use crate::auth::user::account::kernel::data::{AuthUserAccount, ValidateAuthUserAccountError};

impl RegisterAuthUserAccountFieldsExtract for RegisterAuthUserAccountRequestPb {
    fn convert(self) -> Result<AuthUserAccount, ValidateAuthUserAccountError> {
        self.data.try_into()
    }
}
