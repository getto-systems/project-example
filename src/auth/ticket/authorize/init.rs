mod proxy_call;
mod request;

use crate::{
    auth::x_outside_feature::feature::{AuthOutsideDecodingKey, AuthServiceOutsideFeature},
    x_outside_feature::{auth::feature::AuthAppFeature, data::RequestId},
};

use crate::auth::{
    kernel::init::clock::ChronoAuthClock,
    ticket::{
        authorize::init::proxy_call::TonicClarifyAuthorizeTokenProxyCall,
        kernel::init::{
            ticket_repository::dynamodb::DynamoDbAuthTicketRepository,
            token::authorize::decoder::JwtAuthorizeTokenDecoder,
        },
    },
    user::kernel::init::user_repository::dynamodb::DynamoDbAuthUserRepository,
};

use crate::auth::ticket::authorize::action::{
    ClarifyAuthorizeTokenAction, ClarifyAuthorizeTokenMaterial,
};

use crate::auth::ticket::authorize::method::AuthorizeWithTokenInfra;

use crate::auth::ticket::authorize::proxy::AuthorizeInfra;

pub struct ActiveAuthorizeInfra<'a> {
    proxy_call: TonicClarifyAuthorizeTokenProxyCall<'a>,
}

impl<'a> ActiveAuthorizeInfra<'a> {
    pub(in crate::auth) fn from_auth(feature: &'a AuthAppFeature, request_id: RequestId) -> Self {
        Self {
            proxy_call: TonicClarifyAuthorizeTokenProxyCall::new(&feature.service, request_id),
        }
    }
    pub fn from_service(feature: &'a AuthServiceOutsideFeature, request_id: RequestId) -> Self {
        Self {
            proxy_call: TonicClarifyAuthorizeTokenProxyCall::new(&feature.service, request_id),
        }
    }
}

impl<'a> AuthorizeInfra for ActiveAuthorizeInfra<'a> {
    type ProxyCall = TonicClarifyAuthorizeTokenProxyCall<'a>;

    fn proxy_call(&self) -> &Self::ProxyCall {
        &self.proxy_call
    }
}

pub struct ActiveAuthorizeWithTokenInfra<'a> {
    token_decoder: JwtAuthorizeTokenDecoder<'a>,
}

impl<'a> ActiveAuthorizeWithTokenInfra<'a> {
    pub fn new(decoding_key: &'a AuthOutsideDecodingKey) -> Self {
        Self {
            token_decoder: JwtAuthorizeTokenDecoder::new(decoding_key),
        }
    }
}

impl<'a> AuthorizeWithTokenInfra for ActiveAuthorizeWithTokenInfra<'a> {
    type TokenDecoder = JwtAuthorizeTokenDecoder<'a>;

    fn token_decoder(&self) -> &Self::TokenDecoder {
        &self.token_decoder
    }
}

pub struct ActiveClarifyAuthorizeTokenMaterial<'a> {
    authorize_with_token: ActiveAuthorizeWithTokenInfra<'a>,
    clock: ChronoAuthClock,
    ticket_repository: DynamoDbAuthTicketRepository<'a>,
    user_repository: DynamoDbAuthUserRepository<'a>,
}

impl<'a> ActiveClarifyAuthorizeTokenMaterial<'a> {
    pub fn action(feature: &'a AuthAppFeature) -> ClarifyAuthorizeTokenAction<Self> {
        ClarifyAuthorizeTokenAction::with_material(Self {
            authorize_with_token: ActiveAuthorizeWithTokenInfra::new(&feature.decoding_key),
            clock: ChronoAuthClock::new(),
            ticket_repository: DynamoDbAuthTicketRepository::new(&feature.store),
            user_repository: DynamoDbAuthUserRepository::new(&feature.store),
        })
    }
}

impl<'a> ClarifyAuthorizeTokenMaterial for ActiveClarifyAuthorizeTokenMaterial<'a> {
    type AuthorizeWithToken = ActiveAuthorizeWithTokenInfra<'a>;
    type Clock = ChronoAuthClock;
    type TicketRepository = DynamoDbAuthTicketRepository<'a>;
    type UserRepository = DynamoDbAuthUserRepository<'a>;

    fn authorize_with_token(&self) -> &Self::AuthorizeWithToken {
        &self.authorize_with_token
    }
    fn clock(&self) -> &Self::Clock {
        &self.clock
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
    pub use super::request::test::*;

    use crate::auth::{
        kernel::init::clock::test::StaticChronoAuthClock,
        ticket::{
            authorize::init::proxy_call::test::StaticClarifyAuthorizeTokenProxyCall,
            kernel::init::{
                ticket_repository::memory::MemoryAuthTicketRepository,
                token::authorize::decoder::test::StaticAuthorizeTokenDecoder,
            },
        },
        user::kernel::init::user_repository::memory::MemoryAuthUserRepository,
    };

    use crate::auth::ticket::authorize::action::ClarifyAuthorizeTokenMaterial;

    use crate::auth::ticket::authorize::method::AuthorizeWithTokenInfra;

    use crate::auth::ticket::authorize::proxy::AuthorizeInfra;

    use crate::auth::{ticket::kernel::data::AuthTicket, user::kernel::data::AuthUserId};

    pub struct StaticAuthorizeInfra {
        proxy_call: StaticClarifyAuthorizeTokenProxyCall,
    }

    impl StaticAuthorizeInfra {
        pub fn standard() -> Self {
            Self {
                proxy_call: StaticClarifyAuthorizeTokenProxyCall::new(AuthUserId::restore(
                    "USER-ID".to_owned(),
                )),
            }
        }

        pub(in crate::auth) fn new(user_id: AuthUserId) -> Self {
            Self {
                proxy_call: StaticClarifyAuthorizeTokenProxyCall::new(user_id),
            }
        }
    }

    impl AuthorizeInfra for StaticAuthorizeInfra {
        type ProxyCall = StaticClarifyAuthorizeTokenProxyCall;

        fn proxy_call(&self) -> &Self::ProxyCall {
            &self.proxy_call
        }
    }

    pub struct StaticAuthorizeWithTokenInfra {
        pub token_decoder: StaticAuthorizeTokenDecoder,
    }

    impl StaticAuthorizeWithTokenInfra {
        pub fn for_common_test() -> Self {
            Self {
                token_decoder: StaticAuthorizeTokenDecoder::Valid(AuthTicket::standard()),
            }
        }
    }

    impl AuthorizeWithTokenInfra for StaticAuthorizeWithTokenInfra {
        type TokenDecoder = StaticAuthorizeTokenDecoder;

        fn token_decoder(&self) -> &Self::TokenDecoder {
            &self.token_decoder
        }
    }

    pub struct StaticClarifyAuthorizeTokenMaterial<'a> {
        pub authorize_with_token: StaticAuthorizeWithTokenInfra,
        pub clock: StaticChronoAuthClock,
        pub ticket_repository: MemoryAuthTicketRepository<'a>,
        pub user_repository: MemoryAuthUserRepository<'a>,
    }

    impl<'a> ClarifyAuthorizeTokenMaterial for StaticClarifyAuthorizeTokenMaterial<'a> {
        type AuthorizeWithToken = StaticAuthorizeWithTokenInfra;
        type Clock = StaticChronoAuthClock;
        type TicketRepository = MemoryAuthTicketRepository<'a>;
        type UserRepository = MemoryAuthUserRepository<'a>;

        fn authorize_with_token(&self) -> &Self::AuthorizeWithToken {
            &self.authorize_with_token
        }
        fn clock(&self) -> &Self::Clock {
            &self.clock
        }
        fn ticket_repository(&self) -> &Self::TicketRepository {
            &self.ticket_repository
        }
        fn user_repository(&self) -> &Self::UserRepository {
            &self.user_repository
        }
    }
}
