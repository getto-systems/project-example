mod proxy_service;

use actix_web::HttpRequest;

use crate::auth::x_outside_feature::remote::api::feature::AuthOutsideFeature;

use crate::auth::ticket::{
    logout::remote::proxy::init::proxy_service::ProxyService,
    validate::init::ValidateTicketMetadataStruct,
};

use crate::auth::proxy::action::{AuthProxyAction, AuthProxyMaterial};

pub struct LogoutProxyStruct<'a> {
    validate: ValidateTicketMetadataStruct<'a>,
    proxy_service: ProxyService<'a>,
}

impl<'a> LogoutProxyStruct<'a> {
    pub fn action(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> AuthProxyAction<Self> {
        AuthProxyAction::with_material(Self {
            validate: ValidateTicketMetadataStruct::new(&feature.decoding_key, request),
            proxy_service: ProxyService::new(&feature.service, request_id),
        })
    }
}

#[async_trait::async_trait]
impl<'a> AuthProxyMaterial for LogoutProxyStruct<'a> {
    type Validate = ValidateTicketMetadataStruct<'a>;
    type ProxyService = ProxyService<'a>;

    fn extract(self) -> (Self::Validate, Self::ProxyService) {
        (self.validate, self.proxy_service)
    }
}
