mod modify;

use actix_web::HttpRequest;

use crate::auth::x_outside_feature::feature::AuthProxyOutsideFeature;

use crate::auth::{
    ticket::validate::init::ValidateApiMetadataStruct,
    user::account::modify::proxy::init::modify::ModifyUserProxyService,
};

use crate::auth::proxy::action::{AuthProxyAction, AuthProxyMaterial};

pub struct ModifyAuthUserAccountProxyStruct<'a> {
    validate: ValidateApiMetadataStruct<'a>,
    proxy_service: ModifyUserProxyService<'a>,
}

impl<'a> ModifyAuthUserAccountProxyStruct<'a> {
    pub fn action(
        feature: &'a AuthProxyOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
        body: String,
    ) -> AuthProxyAction<Self> {
        AuthProxyAction::with_material(Self {
            validate: ValidateApiMetadataStruct::new(&feature.decoding_key, request),
            proxy_service: ModifyUserProxyService::new(&feature.service, request_id, body),
        })
    }
}

#[async_trait::async_trait]
impl<'a> AuthProxyMaterial for ModifyAuthUserAccountProxyStruct<'a> {
    type Validate = ValidateApiMetadataStruct<'a>;
    type ProxyService = ModifyUserProxyService<'a>;

    fn extract(self) -> (Self::Validate, Self::ProxyService) {
        (self.validate, self.proxy_service)
    }
}
