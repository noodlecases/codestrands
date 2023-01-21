pub mod error;

pub type Result<T, E = error::CodestrandsError> = core::result::Result<T, E>;
