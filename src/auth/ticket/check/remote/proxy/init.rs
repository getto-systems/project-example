mod proxy_service;

use actix_web::HttpRequest;

use crate::auth::remote::x_outside_feature::api::feature::AuthOutsideFeature;

use crate::auth::ticket::{
    check::remote::proxy::init::proxy_service::ProxyService,
    remote::validate::init::ValidateTicketMetadataStruct,
};

use crate::auth::remote::proxy::action::{AuthProxyAction, AuthProxyMaterial};

pub struct CheckAuthTicketProxyStruct<'a> {
    validate: ValidateTicketMetadataStruct<'a>,
    proxy_service: ProxyService<'a>,
}

impl<'a> CheckAuthTicketProxyStruct<'a> {
    pub fn action(
        feature: &'a AuthOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> AuthProxyAction<Self> {
        AuthProxyAction::with_material(Self {
            validate: ValidateTicketMetadataStruct::new(&feature.decoding_key, request),
            proxy_service: ProxyService::new(feature, request_id),
        })
    }
}

#[async_trait::async_trait]
impl<'a> AuthProxyMaterial for CheckAuthTicketProxyStruct<'a> {
    type Validate = ValidateTicketMetadataStruct<'a>;
    type ProxyService = ProxyService<'a>;

    fn extract(self) -> (Self::Validate, Self::ProxyService) {
        (self.validate, self.proxy_service)
    }
}
