use prost::Message;

use crate::auth::password::_api::y_protobuf::api::ChangePasswordPb;

use crate::z_details::_api::message::helper::{decode_base64, invalid_protobuf};

use crate::auth::password::{
    _api::change::infra::ChangePasswordRequestDecoder,
    _common::change::infra::ChangePasswordFieldsExtract,
};

use crate::z_details::_api::message::data::MessageError;

pub struct ProstChangePasswordRequestDecoder {
    body: String,
}

impl ProstChangePasswordRequestDecoder {
    pub const fn new(body: String) -> Self {
        Self { body }
    }
}

impl ChangePasswordRequestDecoder for ProstChangePasswordRequestDecoder {
    fn decode(self) -> Result<ChangePasswordFieldsExtract, MessageError> {
        let message =
            ChangePasswordPb::decode(decode_base64(self.body)?).map_err(invalid_protobuf)?;

        Ok(ChangePasswordFieldsExtract {
            current_password: message.current_password,
            new_password: message.new_password,
        })
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::password::{
        _api::change::infra::ChangePasswordRequestDecoder,
        _common::change::infra::ChangePasswordFieldsExtract,
    };

    use crate::z_details::_api::message::data::MessageError;

    pub struct StaticChangePasswordRequestDecoder {
        pub fields: ChangePasswordFieldsExtract,
    }

    impl ChangePasswordRequestDecoder for StaticChangePasswordRequestDecoder {
        fn decode(self) -> Result<ChangePasswordFieldsExtract, MessageError> {
            Ok(self.fields)
        }
    }
}
