use crate::{Result, U256};


pub const MIN_TICK: i32 = -887272;
pub const MAX_TICK: i32 = 887272;
pub const MIN_SQRT_RATIO: U256 = U256::from_limbs([4_295_128_739, 0, 0, 0]);
/// `get_sqrt_ratio_at_tick(MAX_TICK)`. Equal to
/// `1461446703485210103287273052203988822378723970342`.
pub const MAX_SQRT_RATIO: U256 = U256::from_limbs([
    6_743_328_256_752_651_558,
    17_280_870_778_742_802_505,
    4_294_805_859,
    0,
]);
pub fn get_sqrt_ratio_at_tick(tick: i32) -> Result<U256> {
    unimplemented!()
}

pub fn get_tick_at_sqrt_ratio(sqrt_price_x96: U256) -> Result<i32> {
    unimplemented!()
}

