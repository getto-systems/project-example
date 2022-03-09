mod change_password;
mod override_password;

use actix_web::HttpRequest;

use crate::auth::user::password::change::proxy::init::override_password::OverridePasswordProxyService;
use crate::auth::x_outside_feature::feature::AuthProxyOutsideFeature;

use crate::auth::{
    ticket::validate::init::ValidateApiMetadataStruct,
    user::password::change::proxy::init::change_password::ChangePasswordProxyService,
};

use crate::auth::proxy::action::{AuthProxyAction, AuthProxyMaterial};

pub struct ChangePasswordProxyStruct<'a> {
    validate: ValidateApiMetadataStruct<'a>,
    proxy_service: ChangePasswordProxyService<'a>,
}

impl<'a> ChangePasswordProxyStruct<'a> {
    pub fn action(
        feature: &'a AuthProxyOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
        body: String,
    ) -> AuthProxyAction<Self> {
        AuthProxyAction::with_material(Self {
            validate: ValidateApiMetadataStruct::new(&feature.decoding_key, request),
            proxy_service: ChangePasswordProxyService::new(&feature.service, request_id, body),
        })
    }
}

#[async_trait::async_trait]
impl<'a> AuthProxyMaterial for ChangePasswordProxyStruct<'a> {
    type Validate = ValidateApiMetadataStruct<'a>;
    type ProxyService = ChangePasswordProxyService<'a>;

    fn extract(self) -> (Self::Validate, Self::ProxyService) {
        (self.validate, self.proxy_service)
    }
}

pub struct OverridePasswordProxyStruct<'a> {
    validate: ValidateApiMetadataStruct<'a>,
    proxy_service: OverridePasswordProxyService<'a>,
}

impl<'a> OverridePasswordProxyStruct<'a> {
    pub fn action(
        feature: &'a AuthProxyOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
        body: String,
    ) -> AuthProxyAction<Self> {
        AuthProxyAction::with_material(Self {
            validate: ValidateApiMetadataStruct::new(&feature.decoding_key, request),
            proxy_service: OverridePasswordProxyService::new(&feature.service, request_id, body),
        })
    }
}

#[async_trait::async_trait]
impl<'a> AuthProxyMaterial for OverridePasswordProxyStruct<'a> {
    type Validate = ValidateApiMetadataStruct<'a>;
    type ProxyService = OverridePasswordProxyService<'a>;

    fn extract(self) -> (Self::Validate, Self::ProxyService) {
        (self.validate, self.proxy_service)
    }
}
