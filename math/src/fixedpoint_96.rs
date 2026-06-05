//! Q64.96 fixed-point format.
//! Prices in PCLMM live in Q64.96: a 160-bit-wide number reserved for sqrt(P)
//! where the low 96 bits are fractional. Storing the *square root* (not the
//! price itself) is what makes the swap math linear in `sqrt(P)` , pls refer to the
//! Uniswap V3 whitepaper, section 6.2.
//!
//! Concretely:
//! ```text
//!     realvalue = storedvalue / 2^96
//! ```
//! With 96 fractional bits we can represent ~28 decimal digits of fraction whicxh is
//! more than enough for the 18-decimal precision the curriculum targets.

use crate::U256;
pub const RESOLUTION: u32 = 96;

/// `2^96` as a U256. Equivalently: a `1.0` in Q64.96.
pub const Q96: U256 = U256::from_limbs([0, 1u64 << 32, 0, 0]);
pub const Q128: U256 = U256::from_limbs([0, 0, 1, 0]);