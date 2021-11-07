use prost::Message;

use crate::auth::user::account::remote::y_protobuf::api::SearchAuthUserAccountApiRequestPb;

use crate::z_lib::remote::message::helper::{decode_base64, invalid_protobuf};

use crate::auth::user::account::remote::{
    proxy_search::infra::SearchAuthUserAccountProxyRequestDecoder,
    search::infra::SearchAuthUserAccountFieldsExtract,
};

use crate::z_lib::remote::{message::data::MessageError, search::data::SearchSortExtract};

pub struct RequestDecoder {
    body: String,
}

impl RequestDecoder {
    pub const fn new(body: String) -> Self {
        Self { body }
    }
}

impl SearchAuthUserAccountProxyRequestDecoder for RequestDecoder {
    fn decode(self) -> Result<SearchAuthUserAccountFieldsExtract, MessageError> {
        let message = SearchAuthUserAccountApiRequestPb::decode(decode_base64(self.body)?)
            .map_err(invalid_protobuf)?;

        Ok(SearchAuthUserAccountFieldsExtract {
            offset: message.offset,
            sort: SearchSortExtract {
                key: message.sort_key,
                order: message.sort_order,
            },
            login_id: message.login_id,
        })
    }
}
