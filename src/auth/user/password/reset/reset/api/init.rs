mod request;
mod reset_notifier;

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::auth::{
    kernel::init::clock::ChronoAuthClock,
    ticket::{encode::init::ActiveEncodeAuthTokenInfra, issue::init::ActiveIssueAuthTicketInfra},
    user::{
        kernel::init::user_repository::dynamodb::DynamoDbAuthUserRepository,
        password::{
            kernel::init::password_hasher::Argon2PasswordHasher,
            reset::{
                kernel::init::token::decoder::JwtResetPasswordTokenDecoder,
                reset::init::reset_notifier::EmailResetPasswordNotifier,
            },
        },
    },
};

use crate::auth::user::password::reset::reset::action::{
    ResetPasswordAction, ResetPasswordMaterial,
};

pub struct ActiveResetPasswordMaterial<'a> {
    issue: ActiveIssueAuthTicketInfra<'a>,
    encode: ActiveEncodeAuthTokenInfra<'a>,

    clock: ChronoAuthClock,
    reset_password_repository: DynamoDbAuthUserRepository<'a>,
    token_decoder: JwtResetPasswordTokenDecoder<'a>,
    reset_notifier: EmailResetPasswordNotifier<'a>,
}

impl<'a> ActiveResetPasswordMaterial<'a> {
    pub fn action(feature: &'a AuthAppFeature) -> ResetPasswordAction<Self> {
        ResetPasswordAction::with_material(Self {
            issue: ActiveIssueAuthTicketInfra::new(feature),
            encode: ActiveEncodeAuthTokenInfra::new(feature),

            clock: ChronoAuthClock::new(),
            reset_password_repository: DynamoDbAuthUserRepository::new(&feature.store),
            token_decoder: JwtResetPasswordTokenDecoder::new(&feature.reset_token_key),
            reset_notifier: EmailResetPasswordNotifier::new(&feature.email),
        })
    }
}

impl<'a> ResetPasswordMaterial for ActiveResetPasswordMaterial<'a> {
    type Issue = ActiveIssueAuthTicketInfra<'a>;
    type Encode = ActiveEncodeAuthTokenInfra<'a>;

    type Clock = ChronoAuthClock;
    type ResetPasswordRepository = DynamoDbAuthUserRepository<'a>;
    type PasswordHasher = Argon2PasswordHasher;
    type TokenDecoder = JwtResetPasswordTokenDecoder<'a>;
    type ResetNotifier = EmailResetPasswordNotifier<'a>;

    fn issue(&self) -> &Self::Issue {
        &self.issue
    }
    fn encode(&self) -> &Self::Encode {
        &self.encode
    }

    fn clock(&self) -> &Self::Clock {
        &self.clock
    }
    fn reset_password_repository(&self) -> &Self::ResetPasswordRepository {
        &self.reset_password_repository
    }
    fn token_decoder(&self) -> &Self::TokenDecoder {
        &self.token_decoder
    }
    fn reset_notifier(&self) -> &Self::ResetNotifier {
        &self.reset_notifier
    }
}

#[cfg(test)]
pub mod test {
    pub use super::request::test::*;
    pub use super::reset_notifier::test::*;

    use crate::auth::{
        kernel::init::clock::test::StaticChronoAuthClock,
        ticket::{
            encode::init::test::StaticEncodeAuthTokenInfra,
            issue::init::test::StaticIssueAuthTicketInfra,
        },
        user::{
            kernel::init::user_repository::memory::MemoryAuthUserRepository,
            password::{
                kernel::init::password_hasher::test::PlainPasswordHasher,
                reset::kernel::init::token::decoder::test::StaticResetTokenDecoder,
            },
        },
    };

    use crate::auth::user::password::reset::reset::action::ResetPasswordMaterial;

    pub struct StaticResetPasswordMaterial<'a> {
        pub issue: StaticIssueAuthTicketInfra<'a>,
        pub encode: StaticEncodeAuthTokenInfra<'a>,

        pub clock: StaticChronoAuthClock,
        pub reset_password_repository: MemoryAuthUserRepository<'a>,
        pub token_decoder: StaticResetTokenDecoder,
        pub reset_notifier: StaticResetPasswordNotifier,
    }

    impl<'a> ResetPasswordMaterial for StaticResetPasswordMaterial<'a> {
        type Issue = StaticIssueAuthTicketInfra<'a>;
        type Encode = StaticEncodeAuthTokenInfra<'a>;

        type Clock = StaticChronoAuthClock;
        type ResetPasswordRepository = MemoryAuthUserRepository<'a>;
        type PasswordHasher = PlainPasswordHasher;
        type TokenDecoder = StaticResetTokenDecoder;
        type ResetNotifier = StaticResetPasswordNotifier;

        fn issue(&self) -> &Self::Issue {
            &self.issue
        }
        fn encode(&self) -> &Self::Encode {
            &self.encode
        }

        fn clock(&self) -> &Self::Clock {
            &self.clock
        }
        fn reset_password_repository(&self) -> &Self::ResetPasswordRepository {
            &self.reset_password_repository
        }
        fn token_decoder(&self) -> &Self::TokenDecoder {
            &self.token_decoder
        }
        fn reset_notifier(&self) -> &Self::ResetNotifier {
            &self.reset_notifier
        }
    }
}
