use crate::prelude::*;

pub(crate) fn error(In(result): In<Result<(), AnyError>>) {
    if let Err(err) = result {
        error!("Error: {err:?}")
    }
}
