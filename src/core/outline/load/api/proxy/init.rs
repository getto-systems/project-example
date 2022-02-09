mod proxy_service;

use actix_web::HttpRequest;

use crate::x_outside_feature::proxy::feature::ProxyAppFeature;

use crate::{
    auth::init::ValidateApiMetadataStruct,
    core::outline::load::proxy::init::proxy_service::ProxyService,
};

use crate::auth::proxy::action::{AuthProxyAction, AuthProxyMaterial};

pub struct GetOutlineMenuBadgeProxyStruct<'a> {
    validate: ValidateApiMetadataStruct<'a>,
    proxy_service: ProxyService<'a>,
}

impl<'a> GetOutlineMenuBadgeProxyStruct<'a> {
    pub fn action(
        feature: &'a ProxyAppFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> AuthProxyAction<Self> {
        AuthProxyAction::with_material(Self {
            validate: ValidateApiMetadataStruct::new(&feature.auth.decoding_key, request),
            proxy_service: ProxyService::new(&feature.core.service, request_id),
        })
    }
}

#[async_trait::async_trait]
impl<'a> AuthProxyMaterial for GetOutlineMenuBadgeProxyStruct<'a> {
    type Validate = ValidateApiMetadataStruct<'a>;
    type ProxyService = ProxyService<'a>;

    fn extract(self) -> (Self::Validate, Self::ProxyService) {
        (self.validate, self.proxy_service)
    }
}