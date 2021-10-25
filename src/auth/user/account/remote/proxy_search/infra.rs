use crate::{
    auth::user::account::remote::search::{
        data::SearchUserAccountBasket, infra::SearchUserAccountFieldsExtract,
    },
    z_lib::remote::message::data::MessageError,
};

pub enum SearchUserAccountProxyResponse {
    Success(SearchUserAccountBasket),
}

pub trait SearchUserAccountProxyRequestDecoder {
    fn decode(self) -> Result<SearchUserAccountFieldsExtract, MessageError>;
}
