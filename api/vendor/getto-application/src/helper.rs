use crate::data::MethodResult;

pub fn flatten<S>(result: MethodResult<S>) -> S {
    match result {
        Ok(inner) => inner,
        Err(inner) => inner,
    }
}
