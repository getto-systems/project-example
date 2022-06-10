use crate::auth::user::account::{
    modify::y_protobuf::service::ModifyAuthUserAccountRequestPb,
    y_protobuf::service::AuthUserAccountPb,
};

use crate::auth::user::account::modify::infra::{
    ModifyAuthUserAccountChangesExtract, ModifyAuthUserAccountFieldsExtract,
    ModifyAuthUserAccountRequestDecoder,
};

use crate::auth::user::account::kernel::data::AuthUserAttributesExtract;

pub struct PbModifyAuthUserAccountRequestDecoder {
    request: ModifyAuthUserAccountRequestPb,
}

impl PbModifyAuthUserAccountRequestDecoder {
    pub const fn new(request: ModifyAuthUserAccountRequestPb) -> Self {
        Self { request }
    }
}

impl ModifyAuthUserAccountRequestDecoder for PbModifyAuthUserAccountRequestDecoder {
    fn decode(self) -> ModifyAuthUserAccountFieldsExtract {
        ModifyAuthUserAccountFieldsExtract {
            login_id: self.request.login_id,
            from: self.request.from.map(decode_changes),
            to: self.request.to.map(decode_changes),
        }
    }
}

fn decode_changes(data: AuthUserAccountPb) -> ModifyAuthUserAccountChangesExtract {
    ModifyAuthUserAccountChangesExtract {
        granted_roles: data.granted_roles,
        attrs: AuthUserAttributesExtract { memo: data.memo },
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::user::account::modify::infra::{
        ModifyAuthUserAccountFieldsExtract, ModifyAuthUserAccountRequestDecoder,
    };

    pub enum StaticModifyAuthUserAccountRequestDecoder {
        Valid(ModifyAuthUserAccountFieldsExtract),
    }

    impl ModifyAuthUserAccountRequestDecoder for StaticModifyAuthUserAccountRequestDecoder {
        fn decode(self) -> ModifyAuthUserAccountFieldsExtract {
            match self {
                Self::Valid(fields) => fields,
            }
        }
    }
}
