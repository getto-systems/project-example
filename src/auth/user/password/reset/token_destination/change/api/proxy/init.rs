mod change;

use actix_web::HttpRequest;

use crate::auth::x_outside_feature::feature::AuthProxyOutsideFeature;

use crate::auth::{
    ticket::validate::init::ValidateApiMetadataStruct,
    user::password::reset::token_destination::change::proxy::init::change::ChangeDestinationProxyService,
};

use crate::auth::proxy::action::{AuthProxyAction, AuthProxyMaterial};

pub struct ChangeResetTokenDestinationProxyStruct<'a> {
    validate: ValidateApiMetadataStruct<'a>,
    proxy_service: ChangeDestinationProxyService<'a>,
}

impl<'a> ChangeResetTokenDestinationProxyStruct<'a> {
    pub fn action(
        feature: &'a AuthProxyOutsideFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
        body: String,
    ) -> AuthProxyAction<Self> {
        AuthProxyAction::with_material(Self {
            validate: ValidateApiMetadataStruct::new(&feature.decoding_key, request),
            proxy_service: ChangeDestinationProxyService::new(&feature.service, request_id, body),
        })
    }
}

#[async_trait::async_trait]
impl<'a> AuthProxyMaterial for ChangeResetTokenDestinationProxyStruct<'a> {
    type Validate = ValidateApiMetadataStruct<'a>;
    type ProxyService = ChangeDestinationProxyService<'a>;

    fn extract(self) -> (Self::Validate, Self::ProxyService) {
        (self.validate, self.proxy_service)
    }
}
