mod proxy_service;

use actix_web::HttpRequest;

use crate::auth::remote::x_outside_feature::api::feature::AuthOutsideFeature;

use crate::auth::{
    ticket::remote::validate::init::ValidateApiMetadataStruct,
    user::account::remote::search::proxy::init::proxy_service::ProxyService,
};

use crate::auth::remote::proxy::action::{AuthProxyAction, AuthProxyMaterial};

pub struct SearchAuthUserAccountProxyStruct<'a> {
    validate: ValidateApiMetadataStruct<'a>,
    proxy_service: ProxyService<'a>,
}

impl<'a> SearchAuthUserAccountProxyStruct<'a> {
    pub fn action(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
        body: String,
    ) -> AuthProxyAction<Self> {
        AuthProxyAction::with_material(Self {
            validate: ValidateApiMetadataStruct::new(&feature.decoding_key, request),
            proxy_service: ProxyService::new(&feature.service, request_id, body),
        })
    }
}

#[async_trait::async_trait]
impl<'a> AuthProxyMaterial for SearchAuthUserAccountProxyStruct<'a> {
    type Validate = ValidateApiMetadataStruct<'a>;
    type ProxyService = ProxyService<'a>;

    fn extract(self) -> (Self::Validate, Self::ProxyService) {
        (self.validate, self.proxy_service)
    }
}
