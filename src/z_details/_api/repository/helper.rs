use super::data::{RegisterAttemptResult, RepositoryError};

const REGISTER_TRY_LIMIT: u8 = 10;

pub fn register_attempt<T>(
    register: impl Fn() -> Result<RegisterAttemptResult<T>, RepositoryError>,
) -> Result<T, RepositoryError> {
    let mut count = 0;

    loop {
        count += 1;
        if count > REGISTER_TRY_LIMIT {
            return Err(RepositoryError::InfraError(format!(
                "registration attempts limit exceeded; limit: {}",
                REGISTER_TRY_LIMIT
            )));
        }

        if let RegisterAttemptResult::Success(value) = register()? {
            return Ok(value);
        }
    }
}
