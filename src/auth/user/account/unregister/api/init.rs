pub mod request_decoder;

use tonic::metadata::MetadataMap;

use crate::auth::ticket::kernel::init::ticket_repository::dynamodb::DynamoDbAuthTicketRepository;
use crate::auth::user::account::unregister::y_protobuf::service::UnregisterAuthUserAccountRequestPb;

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::auth::{
    ticket::validate::init::ApiValidateAuthTokenStruct,
    user::{
        account::unregister::init::request_decoder::PbUnregisterAuthUserAccountRequestDecoder,
        kernel::init::user_repository::dynamodb::DynamoDbAuthUserRepository,
    },
};

use super::action::{UnregisterAuthUserAccountAction, UnregisterAuthUserAccountMaterial};

pub struct UnregisterAuthUserAccountFeature<'a> {
    validate: ApiValidateAuthTokenStruct<'a>,
    ticket_repository: DynamoDbAuthTicketRepository<'a>,
    user_repository: DynamoDbAuthUserRepository<'a>,
}

impl<'a> UnregisterAuthUserAccountFeature<'a> {
    pub fn action(
        feature: &'a AuthAppFeature,
        metadata: &'a MetadataMap,
        request: UnregisterAuthUserAccountRequestPb,
    ) -> UnregisterAuthUserAccountAction<PbUnregisterAuthUserAccountRequestDecoder, Self> {
        UnregisterAuthUserAccountAction::with_material(
            PbUnregisterAuthUserAccountRequestDecoder::new(request),
            Self {
                validate: ApiValidateAuthTokenStruct::new(feature, metadata),
                ticket_repository: DynamoDbAuthTicketRepository::new(&feature.store),
                user_repository: DynamoDbAuthUserRepository::new(&feature.store),
            },
        )
    }
}

impl<'a> UnregisterAuthUserAccountMaterial for UnregisterAuthUserAccountFeature<'a> {
    type Validate = ApiValidateAuthTokenStruct<'a>;

    type TicketRepository = DynamoDbAuthTicketRepository<'a>;
    type UserRepository = DynamoDbAuthUserRepository<'a>;

    fn validate(&self) -> &Self::Validate {
        &self.validate
    }

    fn ticket_repository(&self) -> &Self::TicketRepository {
        &self.ticket_repository
    }
    fn user_repository(&self) -> &Self::UserRepository {
        &self.user_repository
    }
}
