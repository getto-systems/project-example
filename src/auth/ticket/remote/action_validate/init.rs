use tonic::metadata::MetadataMap;

use crate::auth::ticket::_common::y_protobuf::service::ValidateApiTokenRequestPb;

use crate::x_outside_feature::remote::auth::feature::AuthAppFeature;

use crate::auth::ticket::remote::validate::init::{
    request_decoder::PbValidateApiTokenRequestDecoder, ApiValidateAuthTokenStruct,
};

use super::action::{ValidateApiTokenAction, ValidateApiTokenMaterial};

use crate::auth::ticket::remote::validate::infra::ValidateApiTokenRequestDecoder;

pub struct ValidateApiTokenFeature<'a> {
    validate: ApiValidateAuthTokenStruct<'a>,
}

impl<'a> ValidateApiTokenFeature<'a> {
    pub fn action(
        feature: &'a AuthAppFeature,
        metadata: &'a MetadataMap,
    ) -> ValidateApiTokenAction<Self> {
        ValidateApiTokenAction::with_material(Self {
            validate: ApiValidateAuthTokenStruct::new(&feature.auth, metadata),
        })
    }
    pub fn request_decoder(
        request: ValidateApiTokenRequestPb,
    ) -> impl ValidateApiTokenRequestDecoder {
        PbValidateApiTokenRequestDecoder::new(request)
    }
}

#[async_trait::async_trait]
impl<'a> ValidateApiTokenMaterial for ValidateApiTokenFeature<'a> {
    type Validate = ApiValidateAuthTokenStruct<'a>;

    fn validate(&self) -> &Self::Validate {
        &self.validate
    }
}
