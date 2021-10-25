use prost::Message;

use crate::auth::user::account::remote::y_protobuf::api::SearchUserAccountApiRequestPb;

use crate::z_lib::remote::message::helper::{decode_base64, invalid_protobuf};

use crate::auth::user::account::remote::{
    proxy_search::infra::SearchUserAccountProxyRequestDecoder,
    search::infra::SearchUserAccountFieldsExtract,
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

impl SearchUserAccountProxyRequestDecoder for RequestDecoder {
    fn decode(self) -> Result<SearchUserAccountFieldsExtract, MessageError> {
        let message = SearchUserAccountApiRequestPb::decode(decode_base64(self.body)?)
            .map_err(invalid_protobuf)?;

        Ok(SearchUserAccountFieldsExtract {
            offset: message.offset,
            sort: SearchSortExtract {
                key: message.sort_key,
                order: message.sort_order,
            },
            login_id: message.login_id,
        })
    }
}
