mod request;

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::auth::{
    ticket::{encode::init::ActiveEncodeAuthTokenInfra, issue::init::ActiveIssueAuthTicketInfra},
    user::{
        kernel::init::user_repository::dynamodb::DynamoDbAuthUserRepository,
        password::kernel::init::password_matcher::Argon2PasswordMatcher,
    },
};

use crate::auth::user::password::authenticate::action::{
    AuthenticateWithPasswordAction, AuthenticateWithPasswordMaterial,
};

pub struct ActiveAuthenticateWithPasswordMaterial<'a> {
    issue: ActiveIssueAuthTicketInfra<'a>,
    encode: ActiveEncodeAuthTokenInfra<'a>,

    password_repository: DynamoDbAuthUserRepository<'a>,
}

impl<'a> ActiveAuthenticateWithPasswordMaterial<'a> {
    pub fn action(feature: &'a AuthAppFeature) -> AuthenticateWithPasswordAction<Self> {
        AuthenticateWithPasswordAction::with_material(Self {
            issue: ActiveIssueAuthTicketInfra::new(feature),
            encode: ActiveEncodeAuthTokenInfra::new(feature),

            password_repository: DynamoDbAuthUserRepository::new(&feature.store),
        })
    }
}

impl<'a> AuthenticateWithPasswordMaterial for ActiveAuthenticateWithPasswordMaterial<'a> {
    type Issue = ActiveIssueAuthTicketInfra<'a>;
    type Encode = ActiveEncodeAuthTokenInfra<'a>;

    type PasswordRepository = DynamoDbAuthUserRepository<'a>;
    type PasswordMatcher = Argon2PasswordMatcher;

    fn issue(&self) -> &Self::Issue {
        &self.issue
    }
    fn encode(&self) -> &Self::Encode {
        &self.encode
    }

    fn password_repository(&self) -> &Self::PasswordRepository {
        &self.password_repository
    }
}

#[cfg(test)]
pub mod test {
    pub use super::request::test::*;

    use crate::auth::{
        ticket::{
            encode::init::test::StaticEncodeAuthTokenInfra,
            issue::init::test::StaticIssueAuthTicketInfra,
        },
        user::{
            kernel::init::user_repository::memory::MemoryAuthUserRepository,
            password::kernel::init::password_matcher::test::PlainPasswordMatcher,
        },
    };

    use crate::auth::user::password::authenticate::action::AuthenticateWithPasswordMaterial;

    pub struct StaticAuthenticateWithPasswordMaterial<'a> {
        pub issue: StaticIssueAuthTicketInfra<'a>,
        pub encode: StaticEncodeAuthTokenInfra<'a>,

        pub password_repository: MemoryAuthUserRepository<'a>,
    }

    impl<'a> AuthenticateWithPasswordMaterial for StaticAuthenticateWithPasswordMaterial<'a> {
        type Issue = StaticIssueAuthTicketInfra<'a>;
        type Encode = StaticEncodeAuthTokenInfra<'a>;

        type PasswordRepository = MemoryAuthUserRepository<'a>;
        type PasswordMatcher = PlainPasswordMatcher;

        fn issue(&self) -> &Self::Issue {
            &self.issue
        }
        fn encode(&self) -> &Self::Encode {
            &self.encode
        }

        fn password_repository(&self) -> &Self::PasswordRepository {
            &self.password_repository
        }
    }
}
