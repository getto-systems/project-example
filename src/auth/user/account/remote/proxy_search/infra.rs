use crate::{
    auth::user::account::remote::search::{
        data::SearchAuthUserAccountBasket, infra::SearchAuthUserAccountFieldsExtract,
    },
    z_lib::remote::message::data::MessageError,
};

pub enum SearchAuthUserAccountProxyResponse {
    Success(SearchAuthUserAccountBasket),
}

pub trait SearchAuthUserAccountProxyRequestDecoder {
    fn decode(self) -> Result<SearchAuthUserAccountFieldsExtract, MessageError>;
}
