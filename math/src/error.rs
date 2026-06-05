//! Errors returned by the math layer.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MathError {
    TickOutOfBounds,

    SqrtPriceOutOfBounds,

    Overflow,

    DivisionByZero,
    
    InvalidLiquidity,
    
    InvalidPriceTarget,
    /// Tick was not aligned to the pool's `tick_spacing`.
    TickNotSpaced,
}

impl core::fmt::Display for MathError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::TickOutOfBounds => "tick is outside [MIN_TICK, MAX_TICK]",
            Self::SqrtPriceOutOfBounds => "sqrt_price_x96 is outside [MIN_SQRT_RATIO, MAX_SQRT_RATIO)",
            Self::Overflow => "arithmetic overflow",
            Self::DivisionByZero => "division by zero",
            Self::InvalidLiquidity => "invalid liquidity",
            Self::InvalidPriceTarget => "price target on wrong side of current price",
            Self::TickNotSpaced => "tick is not a multiple of tick_spacing",
        })
    }
}

#[cfg(feature = "std")]
impl std::error::Error for MathError {}