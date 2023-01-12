use crate::x_outside_feature::{auth::feature::AuthAppFeature, data::RequestId};

use crate::auth::{
    ticket::{
        authorize::init::ActiveAuthorizeInfra,
        kernel::init::ticket_repository::dynamodb::DynamoDbAuthTicketRepository,
    },
    user::kernel::init::user_repository::dynamodb::DynamoDbAuthUserRepository,
};

use crate::auth::user::account::unregister::action::{
    UnregisterAuthUserAccountAction, UnregisterAuthUserAccountMaterial,
};

pub struct ActiveUnregisterAuthUserAccountMaterial<'a> {
    authorize: ActiveAuthorizeInfra<'a>,
    ticket_repository: DynamoDbAuthTicketRepository<'a>,
    user_repository: DynamoDbAuthUserRepository<'a>,
}

impl<'a> ActiveUnregisterAuthUserAccountMaterial<'a> {
    pub fn action(
        feature: &'a AuthAppFeature,
        request_id: RequestId,
    ) -> UnregisterAuthUserAccountAction<Self> {
        UnregisterAuthUserAccountAction::with_material(Self {
            authorize: ActiveAuthorizeInfra::from_auth(feature, request_id),
            ticket_repository: DynamoDbAuthTicketRepository::new(&feature.store),
            user_repository: DynamoDbAuthUserRepository::new(&feature.store),
        })
    }
}

impl<'a> UnregisterAuthUserAccountMaterial for ActiveUnregisterAuthUserAccountMaterial<'a> {
    type Authorize = ActiveAuthorizeInfra<'a>;

    type TicketRepository = DynamoDbAuthTicketRepository<'a>;
    type UserRepository = DynamoDbAuthUserRepository<'a>;

    fn authorize(&self) -> &Self::Authorize {
        &self.authorize
    }

    fn ticket_repository(&self) -> &Self::TicketRepository {
        &self.ticket_repository
    }
    fn user_repository(&self) -> &Self::UserRepository {
        &self.user_repository
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::{
        ticket::{
            authorize::init::test::StaticAuthorizeInfra,
            kernel::init::ticket_repository::memory::MemoryAuthTicketRepository,
        },
        user::kernel::init::user_repository::memory::MemoryAuthUserRepository,
    };

    use crate::auth::user::account::unregister::action::UnregisterAuthUserAccountMaterial;

    use crate::auth::user::account::unregister::infra::{
        UnregisterAuthUserAccountFields, UnregisterAuthUserAccountFieldsExtract,
    };

    use crate::auth::user::login_id::kernel::data::ValidateLoginIdError;

    pub enum StaticUnregisterAuthUserAccountFields {
        Valid(UnregisterAuthUserAccountFields),
    }

    impl UnregisterAuthUserAccountFieldsExtract for StaticUnregisterAuthUserAccountFields {
        fn convert(self) -> Result<UnregisterAuthUserAccountFields, ValidateLoginIdError> {
            match self {
                Self::Valid(fields) => Ok(fields),
            }
        }
    }

    pub struct StaticUnregisterAuthUserAccountMaterial<'a> {
        pub authorize: StaticAuthorizeInfra,
        pub ticket_repository: MemoryAuthTicketRepository<'a>,
        pub user_repository: MemoryAuthUserRepository<'a>,
    }

    impl<'a> UnregisterAuthUserAccountMaterial for StaticUnregisterAuthUserAccountMaterial<'a> {
        type Authorize = StaticAuthorizeInfra;

        type TicketRepository = MemoryAuthTicketRepository<'a>;
        type UserRepository = MemoryAuthUserRepository<'a>;

        fn authorize(&self) -> &Self::Authorize {
            &self.authorize
        }

        fn ticket_repository(&self) -> &Self::TicketRepository {
            &self.ticket_repository
        }
        fn user_repository(&self) -> &Self::UserRepository {
            &self.user_repository
        }
    }
}
