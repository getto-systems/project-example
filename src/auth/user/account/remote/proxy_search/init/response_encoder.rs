use crate::z_lib::remote::message::helper::encode_protobuf_base64;

use crate::auth::user::account::remote::y_protobuf::api::{
    SearchUserAccountApiResponsePb, UserAccountApiPb,
};

use crate::auth::remote::service::proxy::AuthProxyResponseEncoder;

use crate::auth::user::account::remote::proxy_search::infra::SearchUserAccountProxyResponse;

use crate::{
    auth::user::account::remote::proxy_search::data::SearchUserAccountProxyMessage,
    z_lib::remote::message::data::MessageError,
};

pub struct ResponseEncoder;

impl<'a> AuthProxyResponseEncoder<SearchUserAccountProxyResponse, SearchUserAccountProxyMessage>
    for ResponseEncoder
{
    fn encode(
        &self,
        response: SearchUserAccountProxyResponse,
    ) -> Result<SearchUserAccountProxyMessage, MessageError> {
        match response {
            SearchUserAccountProxyResponse::Success(response) => {
                let message = SearchUserAccountApiResponsePb {
                    offset: response.page.offset,
                    limit: response.page.limit,
                    all: response.page.all,
                    users: response
                        .users
                        .into_iter()
                        .map(|user| UserAccountApiPb {
                            login_id: user.login_id.extract(),
                            granted_roles: user.granted_roles.extract().into_iter().collect(),
                        })
                        .collect(),
                    ..Default::default()
                };
                Ok(SearchUserAccountProxyMessage::Success(
                    encode_protobuf_base64(message)?,
                ))
            }
        }
    }
}
