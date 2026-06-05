#![cfg_attr(not(feature = "std"), no_std)]

pub mod bit_math;
pub mod error;
pub mod fixedpoint_96;
pub mod full_math;
pub mod sqrt_price_math;
pub mod swap_math;
pub mod tick_bitmap;
pub mod tick_math;

pub use error::MathError;
pub use ruint::aliases::{U128, U256, U512};

pub type Result<T> = core::result::Result<T, MathError>;