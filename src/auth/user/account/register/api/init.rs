pub mod request_decoder;
pub mod user_id_generator;

use tonic::metadata::MetadataMap;

use crate::auth::user::account::register::init::user_id_generator::UuidAuthUserIdGenerator;
use crate::auth::user::account::register::y_protobuf::service::RegisterAuthUserAccountRequestPb;

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::auth::{
    ticket::validate::init::AuthenticateApiStruct,
    user::{
        account::register::init::request_decoder::PbRegisterAuthUserAccountRequestDecoder,
        kernel::init::user_repository::dynamodb::DynamoDbAuthUserRepository,
    },
};

use super::action::{RegisterAuthUserAccountAction, RegisterAuthUserAccountMaterial};

pub struct RegisterAuthUserAccountFeature<'a> {
    validate: AuthenticateApiStruct<'a>,
    user_id_generator: UuidAuthUserIdGenerator,
    user_repository: DynamoDbAuthUserRepository<'a>,
}

impl<'a> RegisterAuthUserAccountFeature<'a> {
    pub fn action(
        feature: &'a AuthAppFeature,
        metadata: &'a MetadataMap,
        request: RegisterAuthUserAccountRequestPb,
    ) -> RegisterAuthUserAccountAction<PbRegisterAuthUserAccountRequestDecoder, Self> {
        RegisterAuthUserAccountAction::with_material(
            PbRegisterAuthUserAccountRequestDecoder::new(request),
            Self {
                validate: AuthenticateApiStruct::new(feature, metadata),
                user_id_generator: UuidAuthUserIdGenerator::new(),
                user_repository: DynamoDbAuthUserRepository::new(&feature.store),
            },
        )
    }
}

impl<'a> RegisterAuthUserAccountMaterial for RegisterAuthUserAccountFeature<'a> {
    type Authenticate = AuthenticateApiStruct<'a>;

    type UserIdGenerator = UuidAuthUserIdGenerator;
    type UserRepository = DynamoDbAuthUserRepository<'a>;

    fn authenticate(&self) -> &Self::Authenticate {
        &self.validate
    }

    fn user_id_generator(&self) -> &Self::UserIdGenerator {
        &self.user_id_generator
    }
    fn user_repository(&self) -> &Self::UserRepository {
        &self.user_repository
    }
}
