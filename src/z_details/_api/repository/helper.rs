use super::data::{RegisterAttemptResult, RepositoryError};

const REGISTER_TRY_LIMIT: u8 = 10;

pub fn register_attempt<T, E>(
    register: impl Fn() -> Result<RegisterAttemptResult<T>, E>,
    err: impl Fn(RepositoryError) -> E,
) -> Result<T, E> {
    let mut count = 0;

    loop {
        count += 1;
        if count > REGISTER_TRY_LIMIT {
            return Err(err(RepositoryError::InfraError(format!(
                "registration attempts limit exceeded; limit: {}",
                REGISTER_TRY_LIMIT
            ))));
        }

        if let RegisterAttemptResult::Success(value) = register()? {
            return Ok(value);
        }
    }
}
