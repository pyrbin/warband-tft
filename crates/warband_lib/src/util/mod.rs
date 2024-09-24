pub mod math_ext;
pub mod pipe;

#[macro_export]
macro_rules! single {
    ($query:expr) => {
        match $query.get_single() {
            Ok(q) => q,
            _ => {
                #[cfg(debug_assertions)]
                info!("no single result found");
                return;
            }
        }
    };
}

#[macro_export]
macro_rules! single_mut {
    ($query:expr) => {
        match $query.get_single_mut() {
            Ok(q) => q,
            _ => {
                #[cfg(debug_assertions)]
                info!("no single result found");
                return;
            }
        }
    };
}
