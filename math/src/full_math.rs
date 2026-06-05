//! Computes `floor(a * b / d)` (or `ceil`) without losing precision when
//! `a * b` overflows U256. This is used in the core swap math, so it must be correct and efficient.

use crate::{MathError, Result, U256, U512};

/// note that a standard 64 bit cpu cannot store a 256 bits at once, so i'll be using the crate ruint , it breaks down
/// a u256 down  into an array ogf four 64 bit chunks , called limbs..
/// please read the function code carefully, it is very intuitive once you understand the concept of limbs.
fn to_u512(x: U256) -> U512 {
    let l = x.as_limbs();
    U512::from_limbs([l[0], l[1], l[2], l[3], 0, 0, 0, 0])
}

/// Narrow a U512 back into U256, returning `Overflow` if the high four limbs
/// are non-zero.
fn from_u512(x: U512) -> Result<U256> {
    let l = x.as_limbs();
    if l[4] != 0 || l[5] != 0 || l[6] != 0 || l[7] != 0 {
        return Err(MathError::Overflow);
    }
    Ok(U256::from_limbs([l[0], l[1], l[2], l[3]]))
}

/// `floor((a * b) / denominator)`
pub fn mul_div(a: U256, b: U256, denominator: U256) -> Result<U256> {
    if denominator.is_zero() {
        return Err(MathError::DivisionByZero);
    }
    let prod = to_u512(a) * to_u512(b);
    let denom = to_u512(denominator);
    from_u512(prod / denom)
}

/// `ceil((a * b) / denominator)`
pub fn mul_div_rounding_up(a: U256, b: U256, denominator: U256) -> Result<U256> {
    if denominator.is_zero() {
        return Err(MathError::DivisionByZero);
    }
    let prod = to_u512(a) * to_u512(b);
    let denom = to_u512(denominator);
    let quotient = prod / denom;
    let remainder = prod % denom;
    let result = if remainder.is_zero() {
        quotient
    } else {
        quotient + U512::from(1u8)
    };
    from_u512(result)
}

// #[cfg(test)]
// will implement tests later.