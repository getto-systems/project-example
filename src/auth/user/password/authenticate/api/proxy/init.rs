mod proxy_service;

use actix_web::HttpRequest;

use crate::auth::x_outside_feature::feature::AuthProxyOutsideFeature;

use crate::auth::{
    ticket::validate::init::NoValidateMetadataStruct,
    user::password::authenticate::proxy::init::proxy_service::ProxyService,
};

use crate::auth::proxy::action::{AuthProxyAction, AuthProxyMaterial};

pub struct AuthenticatePasswordProxyStruct<'a> {
    validate: NoValidateMetadataStruct<'a>,
    proxy_service: ProxyService<'a>,
}

impl<'a> AuthenticatePasswordProxyStruct<'a> {
    pub fn action(
        feature: &'a AuthProxyOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
        body: String,
    ) -> AuthProxyAction<Self> {
        AuthProxyAction::with_material(Self {
            validate: NoValidateMetadataStruct::new(request),
            proxy_service: ProxyService::new(feature, request_id, body),
        })
    }
}

#[async_trait::async_trait]
impl<'a> AuthProxyMaterial for AuthenticatePasswordProxyStruct<'a> {
    type Validate = NoValidateMetadataStruct<'a>;
    type ProxyService = ProxyService<'a>;

    fn extract(self) -> (Self::Validate, Self::ProxyService) {
        (self.validate, self.proxy_service)
    }
}
