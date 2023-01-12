use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::auth::ticket::{
    authenticate::init::ActiveAuthenticateWithTokenInfra,
    kernel::init::ticket_repository::dynamodb::DynamoDbAuthTicketRepository,
};

use super::action::{LogoutAction, LogoutMaterial};

pub struct ActiveLogoutMaterial<'a> {
    authenticate_with_token: ActiveAuthenticateWithTokenInfra<'a>,
    ticket_repository: DynamoDbAuthTicketRepository<'a>,
}

impl<'a> ActiveLogoutMaterial<'a> {
    pub fn action(feature: &'a AuthAppFeature) -> LogoutAction<Self> {
        LogoutAction::with_material(Self {
            authenticate_with_token: ActiveAuthenticateWithTokenInfra::new(&feature.decoding_key),
            ticket_repository: DynamoDbAuthTicketRepository::new(&feature.store),
        })
    }
}

#[async_trait::async_trait]
impl<'a> LogoutMaterial for ActiveLogoutMaterial<'a> {
    type AuthenticateWithToken = ActiveAuthenticateWithTokenInfra<'a>;
    type TicketRepository = DynamoDbAuthTicketRepository<'a>;

    fn authenticate_with_token(&self) -> &Self::AuthenticateWithToken {
        &self.authenticate_with_token
    }
    fn ticket_repository(&self) -> &Self::TicketRepository {
        &self.ticket_repository
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::ticket::{
        authenticate::init::test::StaticAuthenticateWithTokenInfra,
        kernel::init::ticket_repository::memory::MemoryAuthTicketRepository,
    };

    use crate::auth::ticket::logout::action::LogoutMaterial;

    pub struct StaticLogoutMaterial<'a> {
        pub authenticate_with_token: StaticAuthenticateWithTokenInfra,
        pub ticket_repository: MemoryAuthTicketRepository<'a>,
    }

    impl<'a> LogoutMaterial for StaticLogoutMaterial<'a> {
        type AuthenticateWithToken = StaticAuthenticateWithTokenInfra;
        type TicketRepository = MemoryAuthTicketRepository<'a>;

        fn authenticate_with_token(&self) -> &Self::AuthenticateWithToken {
            &self.authenticate_with_token
        }
        fn ticket_repository(&self) -> &Self::TicketRepository {
            &self.ticket_repository
        }
    }
}
