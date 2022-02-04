mod proxy_service;

use actix_web::HttpRequest;

use crate::x_outside_feature::api::proxy::feature::ProxyAppFeature;

use crate::{
    auth::init::ValidateApiMetadataStruct,
    avail::unexpected_error::notify::api::proxy::init::proxy_service::ProxyService,
};

use crate::auth::proxy::action::{AuthProxyAction, AuthProxyMaterial};

pub struct NotifyUnexpectedErrorProxyStruct<'a> {
    validate: ValidateApiMetadataStruct<'a>,
    proxy_service: ProxyService<'a>,
}

impl<'a> NotifyUnexpectedErrorProxyStruct<'a> {
    pub fn action(
        feature: &'a ProxyAppFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
        body: String,
    ) -> AuthProxyAction<Self> {
        AuthProxyAction::with_material(Self {
            validate: ValidateApiMetadataStruct::new(&feature.auth.decoding_key, request),
            proxy_service: ProxyService::new(&feature.example.service, request_id, body),
        })
    }
}

#[async_trait::async_trait]
impl<'a> AuthProxyMaterial for NotifyUnexpectedErrorProxyStruct<'a> {
    type Validate = ValidateApiMetadataStruct<'a>;
    type ProxyService = ProxyService<'a>;

    fn extract(self) -> (Self::Validate, Self::ProxyService) {
        (self.validate, self.proxy_service)
    }
}
