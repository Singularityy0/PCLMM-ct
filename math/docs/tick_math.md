# Tick Math: sqrt price conversions

## The price grid

Uniswap V3 quantizes the continuous price axis onto a geometric grid:

$$ P(i) = 1.0001^i $$

The ratio between adjacent ticks is exactly `1.0001` (one basis point). The supported range `[MIN_TICK, MAX_TICK]` spans enough scale to cover pairs roughly from $(2^{-128} : 1)$ to $(1 : 2^{128})$.

## Why store sqrt(P)

Concentrated liquidity makes swap math linear in `sqrt(P)` instead of `P`. In whitepaper notation:

$$ \Delta(\sqrt{P}) = \frac{\Delta y}{L}, \quad \Delta(1/\sqrt{P}) = \frac{\Delta x}{L} $$

Storing the square root avoids an expensive `sqrt` per swap step and keeps arithmetic in the fixed-point domain.

## Fixed-point formats

- **Q128.128** is used internally for high precision during exponentiation and multiplication.
- **Q64.96** is used for externally visible sqrt-price values (fits in 160 bits).

The conversion from Q128.128 to Q64.96 is a right shift by 32 bits. When any lower bits are truncated, the result is rounded up to match V3 behavior.

## To implement

`get_sqrt_ratio_at_tick` does not call `pow`. It exploits the binary decomposition of `|tick|`:

$$ 1.0001^{-|tick|/2} = \prod_k 1.0001^{-(2^k)/2} \quad \text{for each set bit } k $$

Each factor $1.0001^{-(2^k)/2}$ is precomputed as a Q128.128 constant. The algorithm:

1. Initialize `ratio` to Q128.128 "one" (`2^128`) or the first constant if bit 0 is set.
2. For each set bit, multiply by the corresponding constant and shift right by 128 to renormalize.
3. If `tick > 0`, invert using `U256::MAX / ratio` to obtain the reciprocal in Q128.128.
4. Shift right by 32 to convert to Q64.96, rounding up when truncated bits are non-zero.

Because the constants and order of operations match V3, results are bit-exact with the Solidity implementation.

## functions summary

- `get_sqrt_ratio_at_tick(tick: i32) -> Result<U256>`
  - Returns `floor(sqrt(1.0001^tick) * 2^96)` as Q64.96.
  - Errors with `TickOutOfBounds` if `|tick|` exceeds `[MIN_TICK, MAX_TICK]`.
  - Rounds up on the final shift to match V3 rounding rules.

- `get_tick_at_sqrt_ratio(sqrt_price_x96: U256) -> Result<i32>`
  - Returns the largest tick `t` such that `get_sqrt_ratio_at_tick(t) <= sqrt_price_x96`.
  - Errors with `SqrtPriceOutOfBounds` if input is outside `[MIN_SQRT_RATIO, MAX_SQRT_RATIO)`.
  - Uses binary search across the full tick range.

## Invariants and edge cases

- `MIN_TICK` and `MAX_TICK` are fixed to match V3 and keep sqrt-price within Q64.96 bounds.
- `MIN_SQRT_RATIO` and `MAX_SQRT_RATIO` are precomputed constants for boundary comparisons.
- The upper bound on `get_tick_at_sqrt_ratio` is strict (exclusive). This mirrors V3 behavior and prevents the swap loop from entering at the absolute maximum.
- Tick spacing (e.g., 1, 10, 60) is enforced by pool configuration, not by this module.


