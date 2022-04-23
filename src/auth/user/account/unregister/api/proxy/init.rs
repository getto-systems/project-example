mod unregister;

use actix_web::HttpRequest;

use crate::auth::x_outside_feature::feature::AuthProxyOutsideFeature;

use crate::auth::{
    ticket::validate::init::ValidateApiMetadataStruct,
    user::account::unregister::proxy::init::unregister::UnregisterUserProxyService,
};

use crate::auth::proxy::action::{AuthProxyAction, AuthProxyMaterial};

pub struct UnregisterAuthUserAccountProxyStruct<'a> {
    validate: ValidateApiMetadataStruct<'a>,
    proxy_service: UnregisterUserProxyService<'a>,
}

impl<'a> UnregisterAuthUserAccountProxyStruct<'a> {
    pub fn action(
        feature: &'a AuthProxyOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
        body: String,
    ) -> AuthProxyAction<Self> {
        AuthProxyAction::with_material(Self {
            validate: ValidateApiMetadataStruct::new(&feature.decoding_key, request),
            proxy_service: UnregisterUserProxyService::new(&feature.service, request_id, body),
        })
    }
}

#[async_trait::async_trait]
impl<'a> AuthProxyMaterial for UnregisterAuthUserAccountProxyStruct<'a> {
    type Validate = ValidateApiMetadataStruct<'a>;
    type ProxyService = UnregisterUserProxyService<'a>;

    fn extract(self) -> (Self::Validate, Self::ProxyService) {
        (self.validate, self.proxy_service)
    }
}
