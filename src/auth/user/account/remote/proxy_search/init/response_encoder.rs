use crate::z_lib::remote::message::helper::encode_protobuf_base64;

use crate::auth::user::account::remote::y_protobuf::api::{
    SearchAuthUserAccountApiResponsePb, AuthUserAccountApiPb,
};

use crate::auth::remote::service::proxy::AuthProxyResponseEncoder;

use crate::auth::user::account::remote::proxy_search::infra::SearchAuthUserAccountProxyResponse;

use crate::{
    auth::user::account::remote::proxy_search::data::SearchAuthUserAccountProxyMessage,
    z_lib::remote::message::data::MessageError,
};

pub struct ResponseEncoder;

impl<'a> AuthProxyResponseEncoder<SearchAuthUserAccountProxyResponse, SearchAuthUserAccountProxyMessage>
    for ResponseEncoder
{
    fn encode(
        &self,
        response: SearchAuthUserAccountProxyResponse,
    ) -> Result<SearchAuthUserAccountProxyMessage, MessageError> {
        match response {
            SearchAuthUserAccountProxyResponse::Success(response) => {
                let message = SearchAuthUserAccountApiResponsePb {
                    offset: response.page.offset,
                    limit: response.page.limit,
                    all: response.page.all,
                    users: response
                        .users
                        .into_iter()
                        .map(|user| AuthUserAccountApiPb {
                            login_id: user.login_id.extract(),
                            granted_roles: user.granted_roles.extract().into_iter().collect(),
                        })
                        .collect(),
                    ..Default::default()
                };
                Ok(SearchAuthUserAccountProxyMessage::Success(
                    encode_protobuf_base64(message)?,
                ))
            }
        }
    }
}
