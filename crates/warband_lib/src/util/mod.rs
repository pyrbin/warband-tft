mod math_trait;
mod pipe;

macro_rules! single {
    ($query:expr) => {
        match $query.get_single() {
            Ok(q) => q,
            _ => {
                return;
            }
        }
    };
}

macro_rules! single_mut {
    ($query:expr) => {
        match $query.get_single_mut() {
            Ok(q) => q,
            _ => {
                return;
            }
        }
    };
}
