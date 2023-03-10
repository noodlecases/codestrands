pub mod auth;
pub mod chrono;
pub mod error;

pub type Result<T, E = error::CodestrandsError> = core::result::Result<T, E>;
