mod detail;

use std::sync::Arc;

use crate::auth::user::password::{
    authenticate::infra::{
        AuthenticateWithPasswordFieldsExtract, AuthenticateWithPasswordInfra,
        AuthenticateWithPasswordLogger, AuthenticateWithPasswordRepository,
    },
    kernel::infra::AuthUserPasswordMatcher,
};

use crate::auth::{
    ticket::kernel::data::AuthenticateSuccess,
    user::{kernel::data::AuthUser, password::authenticate::data::AuthenticateWithPasswordError},
};

pub struct AuthenticateWithPasswordAction<M: AuthenticateWithPasswordInfra> {
    infra: M,
    logger: Arc<dyn AuthenticateWithPasswordLogger>,
}

impl<M: AuthenticateWithPasswordInfra> AuthenticateWithPasswordAction<M> {
    pub async fn authenticate(
        &self,
        fields: impl AuthenticateWithPasswordFieldsExtract,
    ) -> Result<AuthenticateSuccess, AuthenticateWithPasswordError> {
        self.logger.try_to_authenticate_with_password();

        let fields = fields
            .convert()
            .map_err(|err| self.logger.invalid_request(err))?;

        let user_id = self
            .infra
            .repository()
            .lookup_user_id(&fields.login_id)
            .await
            .map_err(|err| self.logger.failed_to_lookup_user(err))?
            .ok_or_else(|| {
                self.logger
                    .user_not_found(AuthenticateWithPasswordError::NotFound(
                        fields.login_id.clone(),
                    ))
            })?;

        let (stored_password, granted) = self
            .infra
            .repository()
            .lookup_password_and_granted(&user_id)
            .await
            .map_err(|err| self.logger.failed_to_lookup_password_and_granted(err))?
            .ok_or_else(|| {
                self.logger
                    .user_not_found(AuthenticateWithPasswordError::NotFound(
                        fields.login_id.clone(),
                    ))
            })?;

        if !self
            .infra
            .password_matcher(fields.plain_password)
            .match_password(stored_password)
            .map_err(|err| self.logger.failed_to_match_password(err))?
        {
            return Err(self
                .logger
                .password_not_matched(AuthenticateWithPasswordError::PasswordNotMatched));
        }

        let user = AuthUser {
            user_id,
            granted: granted.unwrap_or_default(),
        };

        Ok(self.logger.authenticated(AuthenticateSuccess::new(user)))
    }
}
